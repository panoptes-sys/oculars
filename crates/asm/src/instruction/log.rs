//! The `LOGx` instruction.

use crate::{
    AssemblyInstruction, Mnemonic, OpCode, assembly::verify_opcode, fmt::forward_opcode_fmt,
};

/// Append log record with `N` topics.
/// The `N` constant signifies the type of the `LOG` opcode (e.g. `Log<3>` => `LOG3`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Log<const N: u8> {
    /// Private field to disallow struct creation outside of this module.
    _private: (),
}

impl<const N: u8> Log<N> {
    /// Compile time assertion to check if `N` is correct.
    const VALID: () = assert!(
        N <= 4,
        "Only `Log<X>` instructions where `X` <= 4 are supported."
    );

    /// Create a new `LOG` instruction with the specified type.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::Log, OpCode, Mnemonic, AssemblyInstruction};
    /// let log: Log<4> = Log::new();
    /// assert_eq!(log.opcode(), OpCode::Known(Mnemonic::LOG4));
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use oculars_asm::instruction::Log;
    /// let log: Log<10> = Log::new(); // compile fail!
    /// ```
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        () = Self::VALID;
        Self { _private: () }
    }

    /// Returns the mnemonic associated with this instruction.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::Log, Mnemonic};
    /// assert_eq!(Log::<2>::mnemonic(), Mnemonic::LOG2);
    /// ```
    ///
    /// # Panics
    /// Panics if `N` is less than one or greater than 4.
    ///
    /// ```should_panic
    /// # use oculars_asm::{instruction::Log, Mnemonic};
    /// let log = Log::<5>::mnemonic();
    /// ```
    #[must_use]
    pub const fn mnemonic() -> Mnemonic {
        match N {
            0 => Mnemonic::LOG0,
            1 => Mnemonic::LOG1,
            2 => Mnemonic::LOG2,
            3 => Mnemonic::LOG3,
            4 => Mnemonic::LOG4,
            _ => panic!("Only `Log<X>` instructions where `X` <= 4 are supported."),
        }
    }
}

impl<const N: u8> Default for Log<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u8> AssemblyInstruction for Log<N> {
    #[inline]
    fn mnemonic(&self) -> Option<Mnemonic> {
        Some(Self::mnemonic())
    }

    #[inline]
    fn opcode(&self) -> crate::OpCode {
        OpCode::Known(Self::mnemonic())
    }

    fn disassemble(bytes: &[u8]) -> Result<Self, crate::assembly::DisassemblyError> {
        verify_opcode(bytes, Self::mnemonic() as u8)?;
        Ok(Self::new())
    }
}

forward_opcode_fmt!(generic Log, Display, LowerHex, UpperHex, Binary, Octal);
