//! The `PUSHx` instruction.

use crate::{assembly::DisassemblyError, AssemblyInstruction, Mnemonic, OpCode};

/// Place `N`-byte item on stack.
/// The `N` constant signifies the type of the `PUSH` opcode (e.g. `Push<32>` => `PUSH32`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Push<const N: usize> {
    /// The immediate value of this instruction.
    immediate: [u8; N],
}

impl<const N: usize> Push<N> {
    /// Compile time assertion to check if the immediate size is correct.
    const VALID: () = assert!(
        N <= 32,
        "only `Push<X>` instructions where `X` <= 32 are supported"
    );

    /// Create a new `PUSH` instruction with an immediate value.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::Push, OpCode, Mnemonic, AssemblyInstruction};
    /// let push: Push<32> = Push::new([0; 32]);
    /// assert_eq!(push.opcode(), OpCode::Known(Mnemonic::PUSH32));
    /// ```
    ///
    /// This will fail to compile if the size of the immediate value is greater than 32.
    ///
    /// ```compile_fail
    /// # use oculars_asm::instruction::Push;
    /// let push = Push::new([0; 33]); // compile fail!
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(immediate: [u8; N]) -> Self {
        // this produces a compile time error if the size is invalid.
        () = Self::VALID;
        Self { immediate }
    }

    /// Get a reference to the immediate value.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::instruction::Push;
    /// let push = Push::new([1, 3, 3, 7]);
    /// assert_eq!(push.immediate(), &[1, 3, 3, 7]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn immediate(&self) -> &[u8; N] {
        &self.immediate
    }

    /// Return the size of the immediate value.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::instruction::Push;
    /// fn push_size<const N: usize>(push: &Push<N>) {
    ///     assert_eq!(push.immediate_size() as usize, N);
    /// }
    /// ```
    #[expect(
        clippy::cast_possible_truncation,
        reason = "only `Push<X>` instructions where `X` <= 32 are supported"
    )]
    #[must_use]
    #[inline]
    pub const fn immediate_size(&self) -> u8 {
        N as u8
    }

    /// Returns the mnemonic associated with this instruction.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::{instruction::Push, Mnemonic};
    /// assert_eq!(Push::<2>::mnemonic(), Mnemonic::PUSH2);
    /// ```
    ///
    /// # Panics
    /// Panics if `N` is greater than 32.
    ///
    /// ```should_panic
    /// # use oculars_asm::{instruction::Push, Mnemonic};
    /// let push = Push::<77>::mnemonic();
    /// ```
    #[must_use]
    pub const fn mnemonic() -> Mnemonic {
        match N {
            0 => Mnemonic::PUSH0,
            1 => Mnemonic::PUSH1,
            2 => Mnemonic::PUSH2,
            3 => Mnemonic::PUSH3,
            4 => Mnemonic::PUSH4,
            5 => Mnemonic::PUSH5,
            6 => Mnemonic::PUSH6,
            7 => Mnemonic::PUSH7,
            8 => Mnemonic::PUSH8,
            9 => Mnemonic::PUSH9,
            10 => Mnemonic::PUSH10,
            11 => Mnemonic::PUSH11,
            12 => Mnemonic::PUSH12,
            13 => Mnemonic::PUSH13,
            14 => Mnemonic::PUSH14,
            15 => Mnemonic::PUSH15,
            16 => Mnemonic::PUSH16,
            17 => Mnemonic::PUSH17,
            18 => Mnemonic::PUSH18,
            19 => Mnemonic::PUSH19,
            20 => Mnemonic::PUSH20,
            21 => Mnemonic::PUSH21,
            22 => Mnemonic::PUSH22,
            23 => Mnemonic::PUSH23,
            24 => Mnemonic::PUSH24,
            25 => Mnemonic::PUSH25,
            26 => Mnemonic::PUSH26,
            27 => Mnemonic::PUSH27,
            28 => Mnemonic::PUSH28,
            29 => Mnemonic::PUSH29,
            30 => Mnemonic::PUSH30,
            31 => Mnemonic::PUSH31,
            32 => Mnemonic::PUSH32,
            _ => panic!("only `Push<X>` instructions where `X` <= 32 are supported"),
        }
    }
}

impl<const N: usize> AssemblyInstruction for Push<N> {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Self::mnemonic())
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "only `Push<X>` instructions where `X` <= 32 are supported"
    )]
    fn immediate_size(&self) -> u8 {
        N as u8
    }

    fn assemble(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(N + 1);

        bytes.push(self.opcode().into_byte());
        bytes.extend(self.immediate());

        bytes
    }

    fn disassemble(bytes: &[u8]) -> Result<Self, DisassemblyError> {
        if bytes.len() < N + 1 {
            return Err(DisassemblyError::UnexpectedLength {
                got: bytes.len(),
                expected: N + 1,
            });
        }

        let opcode = bytes[0];

        if opcode != Self::mnemonic() as u8 {
            return Err(DisassemblyError::UnexpectedOpcode {
                got: opcode,
                expected: Self::mnemonic() as u8,
            });
        }

        // this slice indexing should not panic because of the length check at the top.
        Ok(Self::new(bytes[1..=N].try_into().expect(
            "the subslice length matches the expected immediate value length",
        )))
    }
}

/// Implements formatting for the [`Push`] instruction by specifying the format of the opcode and
/// the immediate value.
macro_rules! impl_push_fmt {
    ($fmt: ident, $opcode_fmt: literal, $byte_fmt: literal) => {
        impl<const N: usize> std::fmt::$fmt for Push<N> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $opcode_fmt, self.opcode())?;

                for byte in self.immediate() {
                    write!(f, $byte_fmt, byte)?;
                }

                Ok(())
            }
        }
    };
    ($fmt: ident, $opcode_fmt: literal) => {
        impl_push_fmt!($fmt, $opcode_fmt, $opcode_fmt);
    };
}

impl_push_fmt!(Display, "{} 0x", "{:02X}");
impl_push_fmt!(LowerHex, "{:02x}", "{:02x}");
impl_push_fmt!(UpperHex, "{:02X}", "{:02X}");
impl_push_fmt!(Binary, "{:08b}", "{:08b}");
// octal?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_is_sane() {
        let push = Push::new([1, 2, 3, 4]);
        assert_eq!(push.immediate_size(), 4);
        assert_eq!(push.immediate(), &[1, 2, 3, 4]);
        assert_eq!(push.opcode(), Mnemonic::PUSH4);
    }

    #[test]
    fn push_fmt_is_sane() {
        let push = Push::new([0xA, 0xB, 0xC]);
        assert_eq!(format!("{push:?}"), "Push { immediate: [10, 11, 12] }");
        assert_eq!(format!("{push}"), "PUSH3 0x0A0B0C");
        assert_eq!(format!("{push:x}"), "620a0b0c");
        assert_eq!(format!("{push:X}"), "620A0B0C");
        let push = Push::new([0b10, 0b11]);
        assert_eq!(format!("{push:b}"), "011000010000001000000011");
    }
}
