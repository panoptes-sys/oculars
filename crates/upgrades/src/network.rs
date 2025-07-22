//! Ethereum network upgrades.

use chains::Chain;

/// Ethereum network upgrade (hard fork).
pub trait NetworkUpgrade {
    /// Block at which this upgrade gets activated.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{network::NetworkUpgrade, forks::homestead::Homestead};
    /// # use chains::Mainnet;
    ///
    /// assert_eq!(Homestead::activation_block::<Mainnet>(), 1_150_000);
    /// ```
    #[must_use]
    #[inline]
    fn activation_block<C: Chain>() -> u64
    where
        Self: UpgradeActivation<C>,
    {
        Self::block()
    }
}

/// Identifies at what time an upgrade was activated.
pub trait UpgradeActivation<C: Chain> {
    /// Returns the activation block.
    #[must_use]
    fn block() -> u64;
}

// Unless explicitly specified, a network upgrade gets actived on a chain at block 0.
impl<C: Chain, U: NetworkUpgrade> UpgradeActivation<C> for U {
    default fn block() -> u64 {
        0
    }
}
