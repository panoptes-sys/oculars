//! The `SWAPx` instruction.

use crate::{fmt::forward_opcode_fmt, AssemblyInstruction, Mnemonic, OpCode};

/// Exchange 1st and `N+1`th stack items.
/// The `N` constant signifies the type of the `SWAP` opcode (e.g. `Swap<16>` => `SWAP16`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Swap<const N: u8> {
    /// Private field to disallow struct creation outside of this module.
    _private: (),
}

impl<const N: u8> Swap<N> {
    /// Compile time assertion to check if `N` is correct.
    const VALID: () = assert!(
        N >= 1 && N <= 16,
        "only `Swap<X>` instructions where `X` >= 1 && `X` <= 16 are supported"
    );

    /// Create a new `SWAP` instruction with the specified type.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{instruction::Swap, AssemblyInstruction, OpCode, Mnemonic};
    /// let swap: Swap<10> = Swap::new();
    /// assert_eq!(swap.opcode(), OpCode::Known(Mnemonic::SWAP10));
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use oculars_asm2::instruction::Swap;
    /// let swap: Swap<30> = Swap::new(); // compile fail!
    /// ```
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        () = Self::VALID;
        Self { _private: () }
    }
}

impl<const N: u8> Default for Swap<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u8> AssemblyInstruction for Swap<N> {
    fn opcode(&self) -> OpCode {
        OpCode::Known(match N {
            1 => Mnemonic::SWAP1,
            2 => Mnemonic::SWAP2,
            3 => Mnemonic::SWAP3,
            4 => Mnemonic::SWAP4,
            5 => Mnemonic::SWAP5,
            6 => Mnemonic::SWAP6,
            7 => Mnemonic::SWAP7,
            8 => Mnemonic::SWAP8,
            9 => Mnemonic::SWAP9,
            10 => Mnemonic::SWAP10,
            11 => Mnemonic::SWAP11,
            12 => Mnemonic::SWAP12,
            13 => Mnemonic::SWAP13,
            14 => Mnemonic::SWAP14,
            15 => Mnemonic::SWAP15,
            16 => Mnemonic::SWAP16,
            _ => panic!("only `Swap<X>` instructions where `X` >= 1 && `X` <= 16 are supported"),
        })
    }
}

forward_opcode_fmt!(generic Swap, Display, LowerHex, UpperHex, Binary, Octal);
