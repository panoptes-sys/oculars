//! EIP-1052: EXTCODEHASH opcode.
//!
//! ## Abstract
//! This EIP specifies a new opcode, which returns the keccak256 hash of a contract's code.
//!
//! ## Motivation
//! Many contracts need to perform checks on a contract's bytecode, but do not necessarily need the bytecode itself. For instance, a contract may want to check if another contract's bytecode is one of a set of permitted implementations, or it may perform analyses on code and whitelist any contract with matching bytecode if the analysis passes.
//!
//! Contracts can presently do this using the `EXTCODECOPY` (`0x3c`) opcode, but this is expensive, especially for large contracts, in cases where only the hash is required. As a result, we propose a new opcode, `EXTCODEHASH`, which returns the keccak256 hash of a contract's bytecode.
//!
//! ## Specification
//!
//! A new opcode, `EXTCODEHASH`, is introduced, with number `0x3f`. The `EXTCODEHASH`
//! takes one argument from the stack, zeros the first 96 bits
//! and pushes to the stack the keccak256 hash of the code of the account
//! at the address being the remaining 160 bits.
//!
//! In case the account does not exist or is empty (as defined by [EIP-161](./eip-161.md)) `0` is pushed to the stack.
//!
//! In case the account does not have code the keccak256 hash of empty data
//! (i.e. `c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470`)
//! is pushed to the stack.
//!
//! The gas cost of the `EXTCODEHASH` is 400.
//!
//!
//! ## Rationale
//!
//! As described in the motivation section, this opcode is widely useful, and saves
//! on wasted gas in many cases.
//!
//! The gas cost is the same as the gas cost for the `BALANCE` opcode because the
//! execution of the `EXTCODEHASH` requires the same account lookup as in `BALANCE`.
//!
//! Only the 20 last bytes of the argument are significant (the first 12 bytes are
//! ignored) similarly to the semantics of the `BALANCE` (`0x31`), `EXTCODESIZE` (`0x3b`) and
//! `EXTCODECOPY` (`0x3c`).
//!
//! The `EXTCODEHASH` distinguishes accounts without code and non-existing accounts.
//! This is consistent with the way accounts are represented in the state trie.
//! This also allows smart contracts to check whether an account exists.
//!
//!
//! ## Backwards Compatibility
//!
//! There are no backwards compatibility concerns.
//!
//!
//! ## Test Cases
//!
//! 1. The `EXTCODEHASH` of the account without code is `c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470`
//!    what is the keccak256 hash of empty data.
//! 2. The `EXTCODEHASH` of non-existent account is `0`.
//! 3. The `EXTCODEHASH` of a precompiled contract is either `c5d246...` or `0`.
//! 4. If `EXTCODEHASH` of `A` is `X`, then `EXTCODEHASH` of `A + 2**160` is `X`.
//! 5. The `EXTCODEHASH` of an account that selfdestructed in the current transaction.
//! 6. The `EXTCODEHASH` of an account that selfdestructed and later the selfdestruct has been reverted.
//! 7. The `EXTCODEHASH` of an account created in the current transaction.
//! 8. The `EXTCODEHASH` of an account that has been newly created and later the creation has been reverted.
//! 9. The `EXTCODEHASH` of an account that firstly does not exist and later is empty.
//! 10. The `EXTCODEHASH` of an empty account that is going to be cleared by the state clearing rule.
//!
//! Nick Johnson <arachnid@notdot.net>, Paweł Bylica <pawel@ethereum.org>, "EIP-1052: EXTCODEHASH opcode," Ethereum Improvement Proposals, no. 1052, May 2018. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-1052>.

use asm::instruction::ExtCodeHash;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-1052: EXTCODEHASH opcode.
pub struct Eip1052;

impl Eip for Eip1052 {
    const NUMBER: u32 = 1052;
}

introduces_instructions!(Eip1052, ExtCodeHash);
