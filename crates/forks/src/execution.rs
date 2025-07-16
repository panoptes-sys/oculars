//! Ethereum Execution layer upgrades.

use eips::{Eip, IncludesEip};

/// Ethereum execution layer upgrade.
pub trait ExecutionUpgrade {
    /// Returns whether an EIP is included in an execution upgrade.
    #[must_use]
    #[inline]
    fn includes<E: Eip>() -> bool
    where
        Self: IncludesEip<E>,
    {
        <Self as IncludesEip<E>>::includes_eip()
    }
}
