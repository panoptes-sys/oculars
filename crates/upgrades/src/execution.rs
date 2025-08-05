//! Ethereum Execution layer upgrades.

use asm::AssemblyInstruction;

use crate::eip::{Eip, EipSet};

/// Ethereum execution layer upgrade.
pub trait ExecutionUpgrade {
    /// A set of [`Eip`]s that this execution upgrade includes.
    type EipSet: EipSet;

    /// Returns whether this execution upgrade supports a specific instruction.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{execution::ExecutionUpgrade, forks::{homestead::Homestead, frontier::Frontier}};
    /// # use asm::instruction::DelegateCall;
    /// assert!(!Frontier::supports_instruction::<DelegateCall>());
    /// assert!(Homestead::supports_instruction::<DelegateCall>());
    /// ```
    #[must_use]
    #[inline]
    fn supports_instruction<I: AssemblyInstruction>() -> bool {
        Self::EipSet::supports_instruction::<I>()
    }

    /// Returns whether an EIP is included in this execution upgrade.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{execution::ExecutionUpgrade, forks::{constantinople::Constantinople, petersburg::Petersburg}, eips::eip1283::Eip1283};
    /// assert!(Constantinople::includes::<Eip1283>());
    /// assert!(!Petersburg::includes::<Eip1283>());
    /// ```
    #[must_use]
    #[inline]
    fn includes<E: Eip + 'static>() -> bool {
        Self::EipSet::includes_eip::<E>()
    }
}
