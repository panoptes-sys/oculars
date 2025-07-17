//! EIP-170: Contract code size limit.
//! ### Hard fork
//!
//! Spurious Dragon
//!
//! ### Parameters
//! - `MAX_CODE_SIZE`: `0x6000` (`2**14 + 2**13`)
//! - `FORK_BLKNUM`: 2,675,000
//! - `CHAIN_ID`: 1 (Mainnet)
//!
//! ### Specification
//!
//! If `block.number >= FORK_BLKNUM`, then if contract creation initialization returns data with length of **more than** `MAX_CODE_SIZE` bytes, contract creation fails with an out of gas error.
//!
//! ### Rationale
//!
//! Currently, there remains one slight quadratic vulnerability in Ethereum: when a contract is called, even though the call takes a constant amount of gas, the call can trigger O(n) cost in terms of reading the code from disk, preprocessing the code for VM execution, and also adding O(n) data to the Merkle proof for the block's proof-of-validity. At current gas levels, this is acceptable even if suboptimal. At the higher gas levels that could be triggered in the future, possibly very soon due to dynamic gas limit rules, this would become a greater concern—not nearly as serious as recent denial of service attacks, but still inconvenient especially for future light clients verifying proofs of validity or invalidity. The solution is to put a hard cap on the size of an object that can be saved to the blockchain, and do so non-disruptively by setting the cap at a value slightly higher than what is feasible with current gas limits.

use crate::Eip;

/// Maximum smart contract bytecode size as defined in [EIP-170](`Eip170`).
pub const MAX_CODE_SIZE: usize = 0x6000;

/// EIP-170: Contract code size limit.
pub struct Eip170;

impl Eip for Eip170 {
    const NUMBER: u32 = 170;
}
