//! An unknown instruction.

use crate::{assembly::DisassemblyError, fmt::forward_opcode_fmt, AssemblyInstruction, OpCode};

/// An unidentified instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Unknown {
    /// The unidentified byte.
    byte: u8,
}

impl AssemblyInstruction for Unknown {
    #[inline]
    fn opcode(&self) -> OpCode {
        OpCode::Unknown(self.byte)
    }

    fn disassemble(bytes: &[u8]) -> Result<Self, DisassemblyError> {
        let opcode = *bytes.first().ok_or(DisassemblyError::UnexpectedLength {
            got: 0,
            expected: 1,
        })?;

        Ok(Self { byte: opcode })
    }
}

impl Unknown {
    /// Creates a new unknown instruction with the specified byte.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::Unknown, OpCode, AssemblyInstruction};
    /// let unknown = Unknown::new(0xF);
    /// assert_eq!(unknown.opcode(), OpCode::Unknown(0xF));
    /// ```
    #[must_use]
    #[inline]
    pub fn new(byte: u8) -> Self {
        Self { byte }
    }

    /// Returns the unidentified byte.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::instruction::Unknown;
    /// let unknown = Unknown::new(0xF);
    /// assert_eq!(unknown.byte(), 0xF);
    /// ```
    #[must_use]
    #[inline]
    pub fn byte(&self) -> u8 {
        self.byte
    }
}

forward_opcode_fmt!(Unknown, Display, LowerHex, UpperHex, Binary, Octal);
