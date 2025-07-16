//! Ethereum network upgrades.

use chains::Chain;

/// Ethereum network upgrade (hard fork).
pub trait NetworkUpgrade<C: Chain> {
    /// Block at which this upgrade is active.
    fn activation_block() -> u64;
}
