//! Stack Operations.

use super::InstructionMeta;
use crate::opcode::OpCode;

/// Remove item from stack.
pub struct Pop;

/// Place item on stack.
/// The `N` constant signifies the type of the `PUSH` opcode (e.g. `Push<32>` => `PUSH32`).
pub struct Push<const N: usize>([u8; N]);

impl<const N: usize> Push<N> {
    /// Compile time assertion to check if the size is correct.
    const VALID: () = assert!(N <= 32, "immediate value size cannot be greater than 32");

    /// Create a new `PUSH` instruction with an immediate value.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::instruction::Push;
    /// let push: Push<32> = Push::new([0; 32]);
    /// ```
    ///
    /// This will fail to compile if the size of the immediate value is greater than 32.
    ///
    /// ```compile_fail
    /// # use eva_asm::instruction::Push;
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
    /// # use eva_asm::instruction::Push;
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
    /// # use eva_asm::instruction::Push;
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
    const OPCODE: OpCode = match N {
        0 => OpCode::PUSH0,
        1 => OpCode::PUSH1,
        2 => OpCode::PUSH2,
        3 => OpCode::PUSH3,
        4 => OpCode::PUSH4,
        5 => OpCode::PUSH5,
        6 => OpCode::PUSH6,
        7 => OpCode::PUSH7,
        8 => OpCode::PUSH8,
        9 => OpCode::PUSH9,
        10 => OpCode::PUSH10,
        11 => OpCode::PUSH11,
        12 => OpCode::PUSH12,
        13 => OpCode::PUSH13,
        14 => OpCode::PUSH14,
        15 => OpCode::PUSH15,
        16 => OpCode::PUSH16,
        17 => OpCode::PUSH17,
        18 => OpCode::PUSH18,
        19 => OpCode::PUSH19,
        20 => OpCode::PUSH20,
        21 => OpCode::PUSH21,
        22 => OpCode::PUSH22,
        23 => OpCode::PUSH23,
        24 => OpCode::PUSH24,
        25 => OpCode::PUSH25,
        26 => OpCode::PUSH26,
        27 => OpCode::PUSH27,
        28 => OpCode::PUSH28,
        29 => OpCode::PUSH29,
        30 => OpCode::PUSH30,
        31 => OpCode::PUSH31,
        32 => OpCode::PUSH32,
        _ => unreachable!(),
    };
}

/// Duplicate stack items.
/// The `N` constant signifies the type of the `DUP` opcode (e.g. `Dup<16>` => `DUP16`).
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
    /// # use eva_asm::instruction::Dup;
    /// let dup: Dup<10> = Dup::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use eva_asm::instruction::Dup;
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
    const OPCODE: OpCode = match N {
        1 => OpCode::DUP1,
        2 => OpCode::DUP2,
        3 => OpCode::DUP3,
        4 => OpCode::DUP4,
        5 => OpCode::DUP5,
        6 => OpCode::DUP6,
        7 => OpCode::DUP7,
        8 => OpCode::DUP8,
        9 => OpCode::DUP9,
        10 => OpCode::DUP10,
        11 => OpCode::DUP11,
        12 => OpCode::DUP12,
        13 => OpCode::DUP13,
        14 => OpCode::DUP14,
        15 => OpCode::DUP15,
        16 => OpCode::DUP16,
        _ => unreachable!(),
    };
}

/// Exchange stack items.
/// The `N` constant signifies the type of the `SWAP` opcode (e.g. `Swap<16>` => `SWAP16`).
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
    /// # use eva_asm::instruction::Swap;
    /// let swap: Swap<10> = Swap::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use eva_asm::instruction::Swap;
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
    const OPCODE: OpCode = match N {
        1 => OpCode::SWAP1,
        2 => OpCode::SWAP2,
        3 => OpCode::SWAP3,
        4 => OpCode::SWAP4,
        5 => OpCode::SWAP5,
        6 => OpCode::SWAP6,
        7 => OpCode::SWAP7,
        8 => OpCode::SWAP8,
        9 => OpCode::SWAP9,
        10 => OpCode::SWAP10,
        11 => OpCode::SWAP11,
        12 => OpCode::SWAP12,
        13 => OpCode::SWAP13,
        14 => OpCode::SWAP14,
        15 => OpCode::SWAP15,
        16 => OpCode::SWAP16,
        _ => panic!("invalid Swap type"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_sanity() {
        let push = Push::new([1, 2, 3, 4]);
        assert_eq!(push.size(), 4);
        assert_eq!(push.immediate(), &[1, 2, 3, 4]);
        assert_eq!(push.opcode(), OpCode::PUSH4);
    }
}
