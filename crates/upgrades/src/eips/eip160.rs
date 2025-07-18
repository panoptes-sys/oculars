//! EIP-160: EXP cost increase.
//!
//! ### Hard fork
//! Spurious Dragon
//!
//! ### Parameters
//! - `FORK_BLKNUM`: 2,675,000
//! - `CHAIN_ID`: 1
//!
//! ### Specification
//!
//! If `block.number >= FORK_BLKNUM`, increase the gas cost of EXP from 10 + 10 per byte in the exponent to 10 + 50 per byte in the exponent.
//!
//! ### Rationale
//!
//! Benchmarks suggest that EXP is currently underpriced by a factor of about 4–8.
//!
//! Vitalik Buterin (@vbuterin), "EIP-160: EXP cost increase," Ethereum Improvement Proposals, no. 160, October 2016. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-160>.

use crate::eip::Eip;

/// EIP-160: EXP cost increase.
pub struct Eip160;

impl Eip for Eip160 {
    const NUMBER: u32 = 160;
}
