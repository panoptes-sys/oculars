//! Stack Operations.

use derive_more::Display;

use super::InstructionMeta;
use crate::opcode::{Mnemonic, OpCode};

/// Remove item from stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Pop;

impl InstructionMeta for Pop {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::POP)
    }
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
    /// # use asm::instruction::Push;
    /// let push: Push<32> = Push::new([0; 32]);
    /// ```
    ///
    /// This will fail to compile if the size of the immediate value is greater than 32.
    ///
    /// ```compile_fail
    /// # use asm::instruction::Push;
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
    /// # use asm::instruction::Push;
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
    /// # use asm::instruction::Push;
    /// fn push_size<const N: usize>(push: &Push<N>) {
    ///     assert_eq!(push.size() as usize, N);
    /// }
    /// ```
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "N cannot be greater than 32"
    )]
    pub const fn size(&self) -> u8 {
        // N being less than or equal to 32 is being gated by the `new` function.
        N as u8
    }
}

impl<const N: usize> InstructionMeta for Push<N> {
    fn opcode(&self) -> OpCode {
        match N {
            0 => OpCode::Known(Mnemonic::PUSH0),
            1 => OpCode::Known(Mnemonic::PUSH1),
            2 => OpCode::Known(Mnemonic::PUSH2),
            3 => OpCode::Known(Mnemonic::PUSH3),
            4 => OpCode::Known(Mnemonic::PUSH4),
            5 => OpCode::Known(Mnemonic::PUSH5),
            6 => OpCode::Known(Mnemonic::PUSH6),
            7 => OpCode::Known(Mnemonic::PUSH7),
            8 => OpCode::Known(Mnemonic::PUSH8),
            9 => OpCode::Known(Mnemonic::PUSH9),
            10 => OpCode::Known(Mnemonic::PUSH10),
            11 => OpCode::Known(Mnemonic::PUSH11),
            12 => OpCode::Known(Mnemonic::PUSH12),
            13 => OpCode::Known(Mnemonic::PUSH13),
            14 => OpCode::Known(Mnemonic::PUSH14),
            15 => OpCode::Known(Mnemonic::PUSH15),
            16 => OpCode::Known(Mnemonic::PUSH16),
            17 => OpCode::Known(Mnemonic::PUSH17),
            18 => OpCode::Known(Mnemonic::PUSH18),
            19 => OpCode::Known(Mnemonic::PUSH19),
            20 => OpCode::Known(Mnemonic::PUSH20),
            21 => OpCode::Known(Mnemonic::PUSH21),
            22 => OpCode::Known(Mnemonic::PUSH22),
            23 => OpCode::Known(Mnemonic::PUSH23),
            24 => OpCode::Known(Mnemonic::PUSH24),
            25 => OpCode::Known(Mnemonic::PUSH25),
            26 => OpCode::Known(Mnemonic::PUSH26),
            27 => OpCode::Known(Mnemonic::PUSH27),
            28 => OpCode::Known(Mnemonic::PUSH28),
            29 => OpCode::Known(Mnemonic::PUSH29),
            30 => OpCode::Known(Mnemonic::PUSH30),
            31 => OpCode::Known(Mnemonic::PUSH31),
            32 => OpCode::Known(Mnemonic::PUSH32),
            _ => unreachable!(),
        }
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
    /// # use asm::instruction::Dup;
    /// let dup: Dup<10> = Dup::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use asm::instruction::Dup;
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

impl<const N: u8> InstructionMeta for Dup<N> {
    fn opcode(&self) -> OpCode {
        match N {
            1 => OpCode::Known(Mnemonic::DUP1),
            2 => OpCode::Known(Mnemonic::DUP2),
            3 => OpCode::Known(Mnemonic::DUP3),
            4 => OpCode::Known(Mnemonic::DUP4),
            5 => OpCode::Known(Mnemonic::DUP5),
            6 => OpCode::Known(Mnemonic::DUP6),
            7 => OpCode::Known(Mnemonic::DUP7),
            8 => OpCode::Known(Mnemonic::DUP8),
            9 => OpCode::Known(Mnemonic::DUP9),
            10 => OpCode::Known(Mnemonic::DUP10),
            11 => OpCode::Known(Mnemonic::DUP11),
            12 => OpCode::Known(Mnemonic::DUP12),
            13 => OpCode::Known(Mnemonic::DUP13),
            14 => OpCode::Known(Mnemonic::DUP14),
            15 => OpCode::Known(Mnemonic::DUP15),
            16 => OpCode::Known(Mnemonic::DUP16),
            _ => unreachable!(),
        }
    }
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
    /// # use asm::instruction::Swap;
    /// let swap: Swap<10> = Swap::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use asm::instruction::Swap;
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

impl<const N: u8> InstructionMeta for Swap<N> {
    fn opcode(&self) -> OpCode {
        match N {
            1 => OpCode::Known(Mnemonic::SWAP1),
            2 => OpCode::Known(Mnemonic::SWAP2),
            3 => OpCode::Known(Mnemonic::SWAP3),
            4 => OpCode::Known(Mnemonic::SWAP4),
            5 => OpCode::Known(Mnemonic::SWAP5),
            6 => OpCode::Known(Mnemonic::SWAP6),
            7 => OpCode::Known(Mnemonic::SWAP7),
            8 => OpCode::Known(Mnemonic::SWAP8),
            9 => OpCode::Known(Mnemonic::SWAP9),
            10 => OpCode::Known(Mnemonic::SWAP10),
            11 => OpCode::Known(Mnemonic::SWAP11),
            12 => OpCode::Known(Mnemonic::SWAP12),
            13 => OpCode::Known(Mnemonic::SWAP13),
            14 => OpCode::Known(Mnemonic::SWAP14),
            15 => OpCode::Known(Mnemonic::SWAP15),
            16 => OpCode::Known(Mnemonic::SWAP16),
            _ => panic!("invalid Swap type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_sanity() {
        let push = Push::new([1, 2, 3, 4]);
        assert_eq!(push.size(), 4);
        assert_eq!(push.immediate(), &[1, 2, 3, 4]);
        assert_eq!(push.opcode(), Mnemonic::PUSH4);
    }
}
