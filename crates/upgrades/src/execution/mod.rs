//! Ethereum Execution layer upgrades.

use asm::{AssemblyInstruction, Mnemonic, OpCode};

use crate::eip_set::EipSet;

pub mod arrow_glacier;
pub mod berlin;
pub mod byzantium;
pub mod cancun;
pub mod constantinople;
pub mod frontier;
pub mod frontier_thawing;
pub mod gray_glacier;
pub mod homestead;
pub mod istanbul;
pub mod london;
pub mod muir_glacier;
pub mod paris;
pub mod petersburg;
pub mod prague;
pub mod shanghai;
pub mod spurious_dragon;
pub mod tangerine_whistle;

/// Ethereum execution layer upgrade.
pub trait ExecutionUpgrade {
    /// A set of [`Eip`]s that this execution upgrade includes.
    type EipSet: EipSet;

    /// Returns [`true`] if this upgrade supports a [`Mnemonic`].
    /// ```
    /// # use oculars_upgrades::execution::{ExecutionUpgrade, homestead::Homestead};
    /// # use asm::Mnemonic;
    /// assert!(Homestead::supports_mnemonic(Mnemonic::DELEGATECALL));
    /// assert!(!Homestead::supports_mnemonic(Mnemonic::CREATE2));
    /// ```
    #[must_use]
    #[inline]
    fn supports_mnemonic(mnemonic: Mnemonic) -> bool {
        Self::EipSet::supports_mnemonic(mnemonic)
    }

    /// Returns [`true`] if this upgrade supports an [`OpCode`].
    /// ```
    /// # use oculars_upgrades::execution::{ExecutionUpgrade, homestead::Homestead};
    /// # use asm::{OpCode, Mnemonic};
    /// assert!(Homestead::supports_opcode(OpCode::Known(Mnemonic::DELEGATECALL)));
    /// assert!(!Homestead::supports_opcode(OpCode::Unknown(0xF)));
    /// ```
    #[must_use]
    #[inline]
    fn supports_opcode(opcode: OpCode) -> bool {
        match opcode {
            OpCode::Known(mnemonic) => Self::supports_mnemonic(mnemonic),
            OpCode::Unknown(_) => false,
        }
    }

    #[must_use]
    #[inline]
    /// Returns [`true`] if this upgrade supports an instruction.
    /// ```
    /// # use oculars_upgrades::execution::{ExecutionUpgrade, homestead::Homestead};
    /// # use asm::instruction::*;
    /// assert!(Homestead::supports_instruction(&DelegateCall));
    /// assert!(!Homestead::supports_instruction(&Create2));
    /// ```
    fn supports_instruction<I: AssemblyInstruction>(instruction: &I) -> bool {
        Self::supports_opcode(instruction.opcode())
    }
}
