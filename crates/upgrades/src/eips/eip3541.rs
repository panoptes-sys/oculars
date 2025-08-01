//! EIP-3541: Reject new contract code starting with the 0xEF byte.
//!
//! ## Abstract
//!
//! Disallow new code starting with the `0xEF` byte to be deployed. Code already existing in the account trie starting with `0xEF` byte is not affected semantically by this change.
//!
//! ## Motivation
//!
//! Contracts conforming to the EVM Object Format (EOF) are going to be validated at deploy time. In order to guarantee that every EOF-formatted contract in the state is valid, we need to prevent already deployed (and not validated) contracts from being recognized as such format. This will be achieved by choosing a byte sequence for the *magic* that doesn't exist in any of the already deployed contracts. To prevent the growth of the search space and to limit the analysis to the contracts existing before this fork, we disallow the starting byte of the format (the first byte of the magic).
//!
//! Should the EVM Object Format proposal not be deployed in the future, the *magic* can be used by other features depending on versioning. In the case versioning becomes obsolete, it is simple to roll this back by allowing contracts starting with the `0xEF` byte to be deployed again.
//!
//! ## Specification
//!
//! After `block.number == HF_BLOCK` new contract creation (via create transaction, `CREATE` or `CREATE2` instructions) results in an exceptional abort if the _code_'s first byte is `0xEF`.
//!
//! ### Remarks
//!
//! The *initcode* is the code executed in the context of the *create* transaction, `CREATE`, or `CREATE2` instructions. The *initcode* returns *code* (via the `RETURN` instruction), which is inserted into the account. See section 7 ("Contract Creation") in the Yellow Paper for more information.
//!
//! The opcode `0xEF` is currently an undefined instruction, therefore: *It pops no stack items and pushes no stack items, and it causes an exceptional abort when executed.* This means *initcode* or already deployed *code* starting with this instruction will continue to abort execution.
//!
//! The exceptional abort due to *code* starting with `0xEF` behaves exactly the same as any other exceptional abort that can occur during *initcode* execution, i.e. in case of abort all gas provided to a `CREATE*` or create transaction is consumed.
//!
//! ## Rationale
//!
//! The `0xEF` byte was chosen because it resembles **E**xecutable **F**ormat.
//!
//! Contracts using unassigned opcodes are generally understood to be at risk of changing semantics. Hence using the unassigned `0xEF` should have lesser effects, than choosing an assigned opcode, such as `0xFD` (`REVERT`), `0xFE` (`INVALID)`, or `0xFF` (`SELFDESTRUCT`). Arguably while such contracts may not be very useful, they are still using valid opcodes.
//!
//! Analysis in May 2021, on `18084433` contracts in state, showed that there are 0 existing contracts starting with the `0xEF` byte, as opposed to 1, 4, and 12 starting with `0xFD`, `0xFE`, and `0xFF`, respectively.
//!
//! ## Test Cases
//!
//! Each test case below may be executed in 3 different contexts:
//! - create transaction (no account code)
//! - `CREATE`, with account code: `0x6000356000523660006000f0151560165760006000fd5b` (Yul code: `mstore(0, calldataload(0)) if iszero(create(0, 0, calldatasize())) { revert(0, 0) }`),
//! - `CREATE2`, with account code: `0x60003560005260003660006000f5151560185760006000fd5b` (Yul code: `mstore(0, calldataload(0)) if iszero(create2(0, 0, calldatasize(), 0)) { revert(0, 0) }`)
//!
//! | Case  | Calldata | Expected result |
//! | -------- | -------- | -------- |
//! | deploy one byte `ef` | `0x60ef60005360016000f3` | new contract not deployed, transaction fails |
//! | deploy two bytes `ef00` | `0x60ef60005360026000f3` | new contract not deployed, transaction fails |
//! | deploy three bytes `ef0000` | `0x60ef60005360036000f3` | new contract not deployed, transaction fails |
//! | deploy 32 bytes `ef00...00` | `0x60ef60005360206000f3` | new contract not deployed, transaction fails |
//! | deploy one byte `fe` | `0x60fe60005360016000f3` | new contract deployed, transaction succeeds |
//!
//! ## Backwards Compatibility
//!
//! This is a breaking change given new code starting with the `0xEF` byte will not be deployable, and contract creation will result in a failure. However, given bytecode is executed starting at its first byte, code deployed with `0xEF` as the first byte is not executable anyway.
//!
//! While this means no currently executable contract is affected, it does rejects deployment of new data contracts starting with the `0xEF` byte.
//!
//! ## Security Considerations
//!
//! The authors are not aware of any security or `DoS` risks posed by this change.
//!
//! Alex Beregszaszi (@axic), Paweł Bylica (@chfast), Andrei Maiboroda (@gumb0), Alexey Akhunov (@`AlexeyAkhunov`), Christian Reitwiessner (@chriseth), Martin Swende (@holiman), "EIP-3541: Reject new contract code starting with the 0xEF byte," Ethereum Improvement Proposals, no. 3541, March 2021. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-3541>.

use crate::eip::Eip;

/// EIP-3541: Reject new contract code starting with the 0xEF byte.
pub struct Eip3541;

impl Eip for Eip3541 {
    const NUMBER: u32 = 3541;
}
