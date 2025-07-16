//! Frontier hard fork.

use chains::ChainMeta;

use crate::HardForkMeta;

/// Frontier hard fork.
pub struct Frontier;

impl<C: ChainMeta> HardForkMeta<C> for Frontier {
    const BLOCK_NUMBER: u64 = 0;
}
