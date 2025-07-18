//! Ethereum Execution layer upgrades.

use asm::instruction::InstructionMeta;

use crate::eip::{Eip, EipSet, IntroducesInstruction};

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
    fn supports_instruction<I: InstructionMeta>() -> bool
    where
        Self::EipSet: IntroducesInstruction<I>,
    {
        Self::EipSet::eip_introduces_instruction()
    }

    /// Returns whether an EIP is included in this execution upgrade.
    // TODO: Add Petersburg EIP removal example here.
    #[must_use]
    #[inline]
    fn includes<E: Eip + 'static>() -> bool {
        Self::EipSet::includes_eip::<E>()
    }
}
