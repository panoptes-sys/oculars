//! EIP-7516: BLOBBASEFEE instruction.
//!
//! ## Abstract
//!
//! Add a `BLOBBASEFEE (0x4a)` instruction that returns the value of the blob base-fee of the current block it is executing in. It is the identical to [EIP-3198](./eip-3198.md) (`BASEFEE` opcode) except that it returns the blob base-fee as per [EIP-4844](./eip-4844.md).
//!
//! ## Motivation
//!
//! The intended use case would be for contracts to get the value of the blob base-fee. This feature enables blob-data users to programmatically account for the blob gas price, eg:
//!
//! - Allow rollup contracts to trustlessly account for blob data usage costs.
//! - Blob gas futures can be implemented based on it which allows for blob users to smooth out data blob costs.
//!
//! ## Specification
//!
//! Add a `BLOBBASEFEE` instruction with opcode `0x4a`, with gas cost `2`.
//!
//! | Op   | Input | Output | Cost |
//! |------|-------|--------|------|
//! | 0x4a | 0     | 1      | 2    |
//!
//! `BLOBBASEFEE` returns the result of the `get_blob_gasprice(header) -> int` function as defined in [EIP-4844 §Gas accounting](./eip-4844.md#gas-accounting).
//!
//! ## Rationale
//!
//! ### Gas cost
//!
//! The value of the blob base-fee is needed to process data-blob transactions. That means its value is already available before running the EVM code.
//! The instruction does not add extra complexity and additional read/write operations, hence the choice of `2` gas cost. This is also identical to [EIP-3198](./eip-3198.md) (`BASEFEE` opcode)'s cost as it just makes available data that is in the header.
//!
//! ## Backwards Compatibility
//!
//! There are no known backward compatibility issues with this instruction.
//!
//! ## Test Cases
//!
//! ### Nominal Case
//!
//! Assume calling `get_blob_gasprice(header)` (as defined in [EIP-4844 §Gas accounting](./eip-4844.md#gas-accounting)) on the current block's header returns `7 wei`:
//! `BLOBBASEFEE` should push the value `7` (left padded byte32) to the stack.
//!
//! Bytecode: `0x4900` (`BLOBBASEFEE, STOP`)
//!
//! | Pc | Op          | Cost | Stack | `RStack` |
//! |----|-------------|------|-------|--------|
//! | 0  | BLOBBASEFEE | 2    | []    | []     |
//! | 1  | STOP        | 0    | [7]   | []     |
//!
//! Output: 0x
//! Consumed gas: `2`
//!
//! ### Comprehensive Test Suite
//!
//! A complete suite of tests can be found [here](https://github.com/ethereum/execution-spec-tests/blob/1983444bbe1a471886ef7c0e82253ffe2a4053e1/tests/cancun/eip7516_blobgasfee/test_blobgasfee_opcode.py).
//!
//! ## Security Considerations
//!
//! The value of the blob base-fee is not sensitive and is publicly accessible in the block header. There are no known security implications with this instruction.
//!
//! Carl Beekhuizen (@carlbeek), "EIP-7516: BLOBBASEFEE instruction," Ethereum Improvement Proposals, no. 7516, September 2023. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-7516>.

use asm::instruction::BlobBaseFee;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-7516: BLOBBASEFEE instruction.
pub struct Eip7516;

impl Eip for Eip7516 {
    const NUMBER: u32 = 7516;
}

introduces_instructions!(Eip7516, BlobBaseFee);
