//! EIP-1014: Skinny CREATE2.
//! ### Specification
//!
//! Adds a new opcode (`CREATE2`) at `0xf5`, which takes 4 stack arguments: `endowment`, `memory_start`, `memory_length`, `salt`. Behaves identically to `CREATE` (`0xf0`), except using `keccak256( 0xff ++ address ++ salt ++ keccak256(init_code))[12:]` instead of the usual sender-and-nonce-hash as the address where the contract is initialized at.
//!
//! The `CREATE2` has the same `gas` schema as `CREATE`, but also an extra `hashcost` of `GSHA3WORD * ceil(len(init_code) / 32)`, to account for the hashing that must be performed. The `hashcost` is deducted at the same time as memory-expansion gas and `CreateGas` is deducted: _before_ evaluation of the resulting address and the execution of `init_code`.
//!
//! - `0xff` is a single byte,
//! - `address` is always `20` bytes,
//! - `salt` is always `32` bytes (a stack item).
//!
//! The preimage for the final hashing round is thus always exactly `85` bytes long.
//!
//! The coredev-call at 2018-08-10 decided to use the formula above.
//!
//!
//! ### Motivation
//!
//! Allows interactions to (actually or counterfactually in channels) be made with addresses that do not exist yet on-chain but can be relied on to only possibly eventually contain code that has been created by a particular piece of init code. Important for state-channel use cases that involve counterfactual interactions with contracts.
//!
//! ### Rationale
//!
//! #### Address formula
//!
//! * Ensures that addresses created with this scheme cannot collide with addresses created using the traditional `keccak256(rlp([sender, nonce]))` formula, as `0xff` can only be a starting byte for RLP for data many petabytes long.
//! * Ensures that the hash preimage has a fixed size,
//!
//! #### Gas cost
//!
//! Since address calculation depends on hashing the `init_code`, it would leave clients open to `DoS` attacks if executions could repeatedly cause hashing of large pieces of `init_code`, since expansion of memory is paid for only once. This EIP uses the same cost-per-word as the `SHA3` opcode.
//!
//! ### Clarifications
//!
//! The `init_code` is the code that, when executed, produces the runtime bytecode that will be placed into the state, and which typically is used by high level languages to implement a 'constructor'.
//!
//! This EIP makes collisions possible. The behaviour at collisions is specified by [EIP-684](https://github.com/ethereum/EIPs/issues/684):
//!
//! > If a contract creation is attempted, due to either a creation transaction or the `CREATE` (or future `CREATE2`) opcode, and the destination address already has either nonzero nonce, or nonempty code, then the creation throws immediately, with exactly the same behavior as would arise if the first byte in the init code were an invalid opcode. This applies retroactively starting from genesis.
//!
//! Specifically, if `nonce` or `code` is nonzero, then the create-operation fails.
//!
//! With EIP-161
//!
//! > Account creation transactions and the `CREATE` operation SHALL, prior to the execution of the initialisation code, increment the nonce over and above its normal starting value by one
//!
//! This means that if a contract is created in a transaction, the `nonce` is immediately non-zero, with the side-effect that a collision within the same transaction will always fail -- even if it's carried out from the `init_code` itself.
//!
//! It should also be noted that `SELFDESTRUCT` (`0xff`) has no immediate effect on `nonce` or `code`, thus a contract cannot be destroyed and recreated within one transaction.
//!
//! ### Examples
//!
//! Example 0
//! * address `0x0000000000000000000000000000000000000000`
//! * salt `0x0000000000000000000000000000000000000000000000000000000000000000`
//! * `init_code` `0x00`
//! * gas (assuming no mem expansion): `32006`
//! * result: `0x4D1A2e2bB4F88F0250f26Ffff098B0b30B26BF38`
//!
//! Example 1
//! * address `0xdeadbeef00000000000000000000000000000000`
//! * salt `0x0000000000000000000000000000000000000000000000000000000000000000`
//! * `init_code` `0x00`
//! * gas (assuming no mem expansion): `32006`
//! * result: `0xB928f69Bb1D91Cd65274e3c79d8986362984fDA3`
//!
//! Example 2
//! * address `0xdeadbeef00000000000000000000000000000000`
//! * salt `0x000000000000000000000000feed000000000000000000000000000000000000`
//! * `init_code` `0x00`
//! * gas (assuming no mem expansion): `32006`
//! * result: `0xD04116cDd17beBE565EB2422F2497E06cC1C9833`
//!
//! Example 3
//! * address `0x0000000000000000000000000000000000000000`
//! * salt `0x0000000000000000000000000000000000000000000000000000000000000000`
//! * `init_code` `0xdeadbeef`
//! * gas (assuming no mem expansion): `32006`
//! * result: `0x70f2b2914A2a4b783FaEFb75f459A580616Fcb5e`
//!
//! Example 4
//! * address `0x00000000000000000000000000000000deadbeef`
//! * salt `0x00000000000000000000000000000000000000000000000000000000cafebabe`
//! * `init_code` `0xdeadbeef`
//! * gas (assuming no mem expansion): `32006`
//! * result: `0x60f3f640a8508fC6a86d45DF051962668E1e8AC7`
//!
//! Example 5
//! * address `0x00000000000000000000000000000000deadbeef`
//! * salt `0x00000000000000000000000000000000000000000000000000000000cafebabe`
//! * `init_code` `0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef`
//! * gas (assuming no mem expansion): `32012`
//! * result: `0x1d8bfDC5D46DC4f61D6b6115972536eBE6A8854C`
//!
//! Example 6
//! * address `0x0000000000000000000000000000000000000000`
//! * salt `0x0000000000000000000000000000000000000000000000000000000000000000`
//! * `init_code` `0x`
//! * gas (assuming no mem expansion): `32000`
//! * result: `0xE33C0C7F7df4809055C3ebA6c09CFe4BaF1BD9e0`
//!
//! Vitalik Buterin (@vbuterin), "EIP-1014: Skinny CREATE2," Ethereum Improvement Proposals, no. 1014, April 2018. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-1014>.

use asm::instruction::Create2;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-1014: Skinny CREATE2.
pub struct Eip1014;

impl Eip for Eip1014 {
    const NUMBER: u32 = 1014;
}

introduces_instructions!(Eip1014, Create2);
