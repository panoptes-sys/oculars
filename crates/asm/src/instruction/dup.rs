//! The `DUPx` instruction.

use crate::{
    AssemblyInstruction, Mnemonic, OpCode,
    assembly::{DisassemblyError, verify_opcode},
    fmt::forward_opcode_fmt,
};

/// Duplicate `N`th stack item.
/// The `N` constant signifies the type of the `DUP` opcode (e.g. `Dup<16>` => `DUP16`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dup<const N: u8> {
    /// Private field to disallow struct creation outside of this module.
    _private: (),
}

impl<const N: u8> Dup<N> {
    /// Compile time assertion to check if `N` is correct.
    const VALID: () = assert!(
        N >= 1 && N <= 16,
        "only `Dup<X>` instructions where `X` >= 1 && `X` <= 16 are supported"
    );

    /// Creates a new `DUP` instruction with the specified type.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::Dup, AssemblyInstruction, OpCode, Mnemonic};
    /// let dup: Dup<10> = Dup::new();
    /// assert_eq!(dup.opcode(), OpCode::Known(Mnemonic::DUP10));
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use oculars_asm::instruction::Dup;
    /// let dup: Dup<30> = Dup::new(); // compile fail!
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
    /// # use oculars_asm::{instruction::Dup, Mnemonic};
    /// assert_eq!(Dup::<2>::mnemonic(), Mnemonic::DUP2);
    /// ```
    ///
    /// # Panics
    /// Panics if `N` is less than one or greater than 16.
    ///
    /// ```should_panic
    /// # use oculars_asm::{instruction::Dup, Mnemonic};
    /// let dup = Dup::<21>::mnemonic();
    /// ```
    #[must_use]
    pub const fn mnemonic() -> Mnemonic {
        match N {
            1 => Mnemonic::DUP1,
            2 => Mnemonic::DUP2,
            3 => Mnemonic::DUP3,
            4 => Mnemonic::DUP4,
            5 => Mnemonic::DUP5,
            6 => Mnemonic::DUP6,
            7 => Mnemonic::DUP7,
            8 => Mnemonic::DUP8,
            9 => Mnemonic::DUP9,
            10 => Mnemonic::DUP10,
            11 => Mnemonic::DUP11,
            12 => Mnemonic::DUP12,
            13 => Mnemonic::DUP13,
            14 => Mnemonic::DUP14,
            15 => Mnemonic::DUP15,
            16 => Mnemonic::DUP16,
            _ => panic!("only `Dup<X>` instructions where `X` >= 1 && `X` <= 16 are supported"),
        }
    }
}

impl<const N: u8> Default for Dup<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u8> AssemblyInstruction for Dup<N> {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Self::mnemonic())
    }

    fn disassemble(bytes: &[u8]) -> Result<Self, DisassemblyError> {
        verify_opcode(bytes, Self::mnemonic() as u8)?;
        Ok(Self::new())
    }
}

forward_opcode_fmt!(generic Dup, Display, LowerHex, UpperHex, Binary, Octal);
