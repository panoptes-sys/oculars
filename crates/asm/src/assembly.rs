//! Instruction assembly information.

use thiserror::Error;

use crate::{Mnemonic, OpCode};

/// An EVM assembly instruction.
pub trait AssemblyInstruction: Sized {
    /// Returns the size of this instruction in bytes.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::{Stop, Push}, AssemblyInstruction};
    /// assert_eq!(Stop.size(), 1);
    /// assert_eq!(Push::<4>::new([0; 4]).size(), 5);
    /// ```
    #[must_use]
    #[inline]
    fn size(&self) -> u8 {
        self.immediate_size() + 1
    }

    /// Returns the size of this instruction's immediate value.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::{Stop, Push}, AssemblyInstruction};
    /// assert_eq!(Stop.immediate_size(), 0);
    /// assert_eq!(Push::<0>::new([]).immediate_size(), 0);
    /// assert_eq!(Push::<32>::new([0; 32]).immediate_size(), 32);
    /// ```
    #[must_use]
    #[inline]
    fn immediate_size(&self) -> u8 {
        0
    }

    /// Returns the instruction's [`OpCode`].
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{OpCode, Mnemonic, instruction::{Stop, Unknown}, AssemblyInstruction};
    /// assert_eq!(Stop.opcode(), OpCode::Known(Mnemonic::STOP));
    /// assert_eq!(Unknown::new(0xF).opcode(), OpCode::Unknown(0xF));
    /// ```
    #[must_use]
    fn opcode(&self) -> OpCode;

    /// Returns the instruction's [`Mnemonic`] if the instruction is not `Unknown`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{OpCode, Mnemonic, instruction::{Stop, Unknown}, AssemblyInstruction};
    /// assert_eq!(Stop.mnemonic(), Some(Mnemonic::STOP));
    /// assert!(Unknown::new(0xF).mnemonic().is_none());
    /// ```
    #[must_use]
    fn mnemonic(&self) -> Option<Mnemonic>;

    /// Returns [`true`] if this instruction is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Push, Gas}};
    /// assert_eq!(Push::new([0; 10]).is_push(), true);
    /// assert_eq!(Gas.is_push(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_push(&self) -> bool {
        self.opcode().is_push()
    }

    /// Returns [`true`] if this instruction is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Dup, Gas}};
    /// assert_eq!(Dup::<10>::new().is_dup(), true);
    /// assert_eq!(Gas.is_dup(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_dup(&self) -> bool {
        self.opcode().is_dup()
    }

    /// Returns [`true`] if this instruction is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Swap, Gas}};
    /// assert_eq!(Swap::<10>::new().is_swap(), true);
    /// assert_eq!(Gas.is_swap(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_swap(&self) -> bool {
        self.opcode().is_swap()
    }

    /// Returns [`true`] if this instruction is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Log, Gas}};
    /// assert_eq!(Log::<3>::new().is_log(), true);
    /// assert_eq!(Gas.is_log(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_log(&self) -> bool {
        self.opcode().is_log()
    }

    /// Returns [`true`] for instructions that terminate execution of the smart contract.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Return, Unknown, Gas}};
    /// assert_eq!(Return.is_terminator(), true);
    /// assert_eq!(Unknown::new(0xF).is_terminator(), true);
    /// assert_eq!(Gas.is_terminator(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_terminator(&self) -> bool {
        self.opcode().is_terminator()
    }

    /// Returns [`true`] if this instruction is a `JUMP`, `JUMPI` or a `JUMPDEST`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Jump, JumpDest, Gas}};
    /// assert_eq!(Jump.is_control_flow(), true);
    /// assert_eq!(JumpDest.is_control_flow(), true);
    /// assert_eq!(Gas.is_control_flow(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_control_flow(&self) -> bool {
        self.opcode().is_control_flow()
    }

    /// Assembles this instruction into its byte representation.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Gas, Push}};
    /// assert_eq!(Gas.assemble(), vec![0x5A]);
    /// assert_eq!(Push::new([0xA, 0xB]).assemble(), vec![0x61, 0xA, 0xB]);
    /// ```
    #[must_use]
    #[inline]
    fn assemble(self) -> Vec<u8> {
        vec![self.opcode().into_byte()]
    }

    /// Disassembles an instruction from a sequence of bytes.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{AssemblyInstruction, instruction::{Gas, Push}};
    /// assert_eq!(Gas::disassemble(&[0x5A]).unwrap(), Gas);
    /// assert_eq!(Push::disassemble(&[0x61, 0xA, 0xB]).unwrap(), Push::<2>::new([0xA, 0xB]));
    /// ```
    ///
    /// # Errors
    /// Returns an error if an unexpected opcode was encountered of if length of the byte sequence
    /// is invalid for the corresponding instruction.
    fn disassemble(bytes: &[u8]) -> Result<Self, DisassemblyError>;
}

/// Errors that can happen during instruction disassembly.
#[derive(Debug, Error)]
pub enum DisassemblyError {
    /// An unexpected opcode was encountered.
    #[error("unexpected opcode: expected `{expected}`, got `{got}`")]
    UnexpectedOpcode {
        /// The received opcode.
        got: u8,
        /// The expected opcode.
        expected: u8,
    },

    /// The length of the byte sequence was not as expected.
    #[error("unexpected byte sequence length: expected `{expected}`, got `{got}`")]
    UnexpectedLength {
        /// Received byte sequence length.
        got: usize,
        /// Expected byte sequence length.
        expected: usize,
    },
}

/// Retrieves the first byte from the `bytes` slice and checks if it matches the expected byte.
///
/// # Errors
/// Returns an error if the byte array contains no elements or if the first byte does not match.
pub(crate) fn verify_opcode(bytes: &[u8], expected: u8) -> Result<(), DisassemblyError> {
    let opcode = *bytes.first().ok_or(DisassemblyError::UnexpectedLength {
        got: 0,
        expected: 1,
    })?;

    if opcode != expected {
        return Err(DisassemblyError::UnexpectedOpcode {
            got: opcode,
            expected,
        });
    }

    Ok(())
}
