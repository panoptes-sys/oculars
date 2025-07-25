//! EIP-3855: PUSH0 instruction.
//!
//! ## Abstract
//!
//! Introduce the `PUSH0` (`0x5f`) instruction, which pushes the constant value 0 onto the stack.
//!
//! ## Motivation
//!
//! Many instructions expect offsets as inputs, which in a number of cases are zero. A good example is the return data parameters of `CALLs`, which are set to zeroes in case the contract prefers using `RETURNDATA*`. This is only one example, but there are many other reasons why a contract would need to push a zero value. They can achieve that today by `PUSH1 0`, which costs 3 gas at runtime, and is encoded as two bytes which means `2 * 200` gas deployment cost.
//!
//! Because of the overall cost many try to use various other instructions to achieve the same effect. Common examples include `PC`, `MSIZE`, `CALLDATASIZE`, `RETURNDATASIZE`, `CODESIZE`, `CALLVALUE`, and `SELFBALANCE`. Some of these cost only 2 gas and are a single byte long, but their value can depend on the context.
//!
//! We have conducted an analysis on Mainnet (block ranges 8,567,259…8,582,058 and 12,205,970…12,817,405), and ~11.5% of all the `PUSH*` instructions executed push a value of zero.
//!
//! The main motivations for this change include:
//! 1. Reducing contract code size.
//! 2. Reducing the risk of contracts (mis)using various instructions as an optimisation measure. Repricing/changing those instructions can be more risky.
//! 3. Reduce the need to use `DUP` instructions for duplicating zeroes.
//!
//! To put the "waste" into perspective, across existing accounts 340,557,331 bytes are wasted on `PUSH1 00` instructions, which means 68,111,466,200 gas was spent to deploy them. In practice a lot of these accounts share identical bytecode with others, so their total stored size in clients is lower, however the deploy time cost must have been paid nevertheless.
//!
//! An example for 2) is changing the behaviour of `RETURNDATASIZE` such that it may not be guaranteed to be zero at the beginning of the call frame.
//!
//! ## Specification
//!
//! The instruction `PUSH0` is introduced at `0x5f`. It has no immediate data, pops no items from the stack, and places a single item with the value 0 onto the stack. The cost of this instruction is 2 gas (aka `base`).
//!
//! ## Rationale
//!
//! ### Gas cost
//!
//! The `base` gas cost is used for instructions which place constant values onto the stack, such as `ADDRESS`, `ORIGIN`, and so forth.
//!
//! ### Opcode
//!
//! `0x5f` means it is in a "contiguous" space with the rest of the `PUSH` implementations and potentially could share the implementation.
//!
//! ## Backwards Compatibility
//!
//! This EIP introduces a new opcode which did not exist previously. Already deployed contracts using this opcode could change their behaviour after this EIP.
//!
//! ## Test Cases
//!
//! - `5F` -- successful execution, stack consist of a single item, set to zero
//! - `5F5F..5F` (1024 times) -- successful execution, stack consists of 1024 items, all set to zero
//! - `5F5F..5F` (1025 times) -- execution aborts due to out of stack
//!
//! ## Security Considerations
//!
//! The authors are not aware of any impact on security. Note that jumpdest-analysis is unaffected, as `PUSH0` has no immediate data bytes.
//!
//! Alex Beregszaszi (@axic), Hugo De la cruz (@hugo-dc), Paweł Bylica (@chfast), "EIP-3855: PUSH0 instruction," Ethereum Improvement Proposals, no. 3855, February 2021. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-3855>.

use asm::instruction::Push;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-3855: PUSH0 instruction.
pub struct Eip3855;

impl Eip for Eip3855 {
    const NUMBER: u32 = 3855;
}

introduces_instructions!(Eip3855, Push<0>);
