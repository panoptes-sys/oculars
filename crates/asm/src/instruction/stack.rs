//! Stack Operations.

use super::{InstructionMeta, KnownInstruction};
use crate::opcode::Mnemonic;
use derive_more::Display;

/// Remove item from stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Pop;

impl KnownInstruction for Pop {
    const MNEMONIC: Mnemonic = Mnemonic::POP;
}

/// Place item on stack.
/// The `N` constant signifies the type of the `PUSH` opcode (e.g. `Push<32>` => `PUSH32`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Push<const N: usize>([u8; N]);

impl<const N: usize> Push<N> {
    /// Compile time assertion to check if the size is correct.
    const VALID: () = assert!(N <= 32, "immediate value size cannot be greater than 32");

    /// Create a new `PUSH` instruction with an immediate value.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::instruction::Push;
    /// let push: Push<32> = Push::new([0; 32]);
    /// ```
    ///
    /// This will fail to compile if the size of the immediate value is greater than 32.
    ///
    /// ```compile_fail
    /// # use oculars_asm::instruction::Push;
    /// let push = Push::new([0; 33]); // compile fail!
    /// ```
    #[must_use]
    pub const fn new(immediate: [u8; N]) -> Self {
        // this produces a compile time error if the size is invalid.
        () = Self::VALID;
        Self(immediate)
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
    pub const fn immediate(&self) -> &[u8; N] {
        &self.0
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
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "N cannot be greater than 32"
    )]
    pub const fn immediate_size(&self) -> u8 {
        // N being less than or equal to 32 is being gated by the `new` function.
        N as u8
    }
}

impl<const N: usize> KnownInstruction for Push<N> {
    const __SIZE: usize = 1 + N;

    const MNEMONIC: Mnemonic = match N {
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
        _ => unreachable!(),
    };
}

impl<const N: usize> Default for Push<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

/// Duplicate stack items.
/// The `N` constant signifies the type of the `DUP` opcode (e.g. `Dup<16>` => `DUP16`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Dup<const N: u8> {
    /// Private field to disallow struct creation outside of this module.
    _private: (),
}

impl<const N: u8> Dup<N> {
    /// Compile time assertion to check if `N` is correct.
    const VALID: () = assert!(N >= 1 && N <= 16, "invalid DUP instruction");

    /// Create a new `DUP` instruction with the specified type.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::instruction::Dup;
    /// let dup: Dup<10> = Dup::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use oculars_asm::instruction::Dup;
    /// let dup: Dup<30> = Dup::new(); // compile fail!
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        () = Self::VALID;
        Self { _private: () }
    }
}

impl<const N: u8> Default for Dup<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u8> KnownInstruction for Dup<N> {
    const MNEMONIC: Mnemonic = match N {
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
        _ => unreachable!(),
    };
}

/// Exchange stack items.
/// The `N` constant signifies the type of the `SWAP` opcode (e.g. `Swap<16>` => `SWAP16`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Swap<const N: u8> {
    /// Private field to disallow struct creation outside of this module.
    _private: (),
}

impl<const N: u8> Swap<N> {
    /// Compile time assertion to check if `N` is correct.
    const VALID: () = assert!(N >= 1 && N <= 16, "invalid SWAP instruction");

    /// Create a new `SWAP` instruction with the specified type.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm::instruction::Swap;
    /// let swap: Swap<10> = Swap::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use oculars_asm::instruction::Swap;
    /// let swap: Swap<30> = Swap::new(); // compile fail!
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        () = Self::VALID;
        Self { _private: () }
    }
}

impl<const N: u8> Default for Swap<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u8> KnownInstruction for Swap<N> {
    const MNEMONIC: Mnemonic = match N {
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
        _ => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_sanity() {
        let push = Push::new([1, 2, 3, 4]);
        assert_eq!(push.immediate_size(), 4);
        assert_eq!(push.immediate(), &[1, 2, 3, 4]);
        assert_eq!(push.opcode(), Mnemonic::PUSH4);
    }
}
