//! EIP-100: Change difficulty adjustment to target mean block time including uncles.
//!
//! ### Specification
//!
//! Currently, the formula to compute the difficulty of a block includes the following logic:
//!
//! ``` python
//! adj_factor = max(1 - ((timestamp - parent.timestamp) // 10), -99)
//! child_diff = int(max(parent.difficulty + (parent.difficulty // BLOCK_DIFF_FACTOR) * adj_factor, min(parent.difficulty, MIN_DIFF)))
//! ...
//! ```
//!
//! If `block.number >= BYZANTIUM_FORK_BLKNUM`, we change the first line to the following:
//!
//! ``` python
//! adj_factor = max((2 if len(parent.uncles) else 1) - ((timestamp - parent.timestamp) // 9), -99)
//! ```
//! ### Rationale
//!
//! This new formula ensures that the difficulty adjustment algorithm targets a constant average rate of blocks produced including uncles, and so ensures a highly predictable issuance rate that cannot be manipulated upward by manipulating the uncle rate. A formula that accounts for the exact number of included uncles:
//! ``` python
//! adj_factor = max(1 + len(parent.uncles) - ((timestamp - parent.timestamp) // 9), -99)
//! ```
//! can be fairly easily seen to be (to within a tolerance of ~3/4194304) mathematically equivalent to assuming that a block with `k` uncles is equivalent to a sequence of `k+1` blocks that all appear with the exact same timestamp, and this is likely the simplest possible way to accomplish the desired effect. But since the exact formula depends on the full block and not just the header, we are instead using an approximate formula that accomplishes almost the same effect but has the benefit that it depends only on the block header (as you can check the uncle hash against the blank hash).
//!
//! Changing the denominator from 10 to 9 ensures that the block time remains roughly the same (in fact, it should decrease by ~3% given the current uncle rate of 7%).
//!
//! Vitalik Buterin (@vbuterin), "EIP-100: Change difficulty adjustment to target mean block time including uncles," Ethereum Improvement Proposals, no. 100, April 2016. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-100>.

use crate::eip::Eip;

/// EIP-100: Change difficulty adjustment to target mean block time including uncles.
pub struct Eip100;

impl Eip for Eip100 {
    const NUMBER: u32 = 100;
}
