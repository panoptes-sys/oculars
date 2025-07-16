//! Frontier Thawing hard fork.

use chains::ChainMeta;

use crate::HardForkMeta;

/// Frontier Thawing hard fork.
pub struct FrontierThawing;

impl<C: ChainMeta> HardForkMeta<C> for FrontierThawing {
    const BLOCK_NUMBER: u64 = 200_000;
}
