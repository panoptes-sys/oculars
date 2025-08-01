//! EIP-140: REVERT instruction.
//!
//! ## Simple Summary
//!
//! The `REVERT` instruction provides a way to stop execution and revert state changes, without consuming all provided gas and with the ability to return a reason.
//!
//! ## Abstract
//!
//! The `REVERT` instruction will stop execution, roll back all state changes done so far and provide a pointer to a memory section, which can be interpreted as an error code or message. While doing so, it will not consume all the remaining gas.
//!
//! ## Motivation
//!
//! Currently this is not possible. There are two practical ways to revert a transaction from within a contract: running out of gas or executing an invalid instruction. Both of these options will consume all remaining gas. Additionally, reverting an EVM execution means that all changes, including LOGs, are lost and there is no way to convey a reason for aborting an EVM execution.
//!
//! ## Specification
//!
//! On blocks with `block.number >= BYZANTIUM_FORK_BLKNUM`, the `REVERT` instruction is introduced at `0xfd`. It expects two stack items, the top item is the `memory_offset` followed by `memory_length`. It does not produce any stack elements because it stops execution.
//!
//! The semantics of `REVERT` with respect to memory and memory cost are identical to those of `RETURN`. The sequence of bytes given by `memory_offset` and `memory_length` is called "error message" in the following.
//!
//! The effect of `REVERT` is that execution is aborted, considered as failed, and state changes are rolled back. The error message will be available to the caller in the returndata buffer and will also be copied to the output area, i.e. it is handled in the same way as the regular return data is handled.
//!
//! The cost of the `REVERT` instruction equals to that of the `RETURN` instruction, i.e. the rollback itself does not consume all gas, the contract only has to pay for memory.
//!
//! In case there is not enough gas left to cover the cost of `REVERT` or there is a stack underflow, the effect of the `REVERT` instruction will equal to that of a regular out of gas exception, i.e. it will consume all gas.
//!
//! In the same way as all other failures, the calling opcode returns `0` on the stack following a `REVERT` opcode in the callee.
//!
//! In case `REVERT` is used in the context of a `CREATE` or `CREATE2` call, no code is deployed, `0` is put on the stack and the error message is available in the returndata buffer.
//!
//! The content of the optionally provided memory section is not defined by this EIP, but is a candidate for another Informational EIP.
//!
//! ## Backwards Compatibility
//!
//! This change has no effect on contracts created in the past unless they contain `0xfd` as an instruction.
//!
//! ## Test Cases
//!
//! ```python
//! 6c726576657274656420646174616000557f726576657274206d657373616765000000000000000000000000000000000000600052600e6000fd
//! ```
//!
//! should:
//! - return `0x726576657274206d657373616765` as `REVERT` data,
//! - the storage at key `0x0` should be left as unset and
//! - use 20024 gas in total.
//!
//! Alex Beregszaszi (@axic), Nikolai Mushegian <nikolai@nexusdev.us>, "EIP-140: REVERT instruction," Ethereum Improvement Proposals, no. 140, February 2017. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-140>.

use asm::instruction::Revert;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-140: REVERT instruction.
pub struct Eip140;

impl Eip for Eip140 {
    const NUMBER: u32 = 140;
}

introduces_instructions!(Eip140, Revert);
