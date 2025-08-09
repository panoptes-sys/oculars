//! Ethereum Execution layer upgrades.

use asm::AssemblyInstruction;

use crate::eip::EipSet;

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
}
