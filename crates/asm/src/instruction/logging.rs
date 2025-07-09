//! Logging operations.

use super::InstructionMeta;
use crate::opcode::OpCode;

/// Append log record.
/// The `N` constant signifies the type of the `LOG` opcode (e.g. `Log<3>` => `LOG3`).
pub struct Log<const N: u8> {
    /// Private field to disallow struct creation outside of this module.
    _private: (),
}

impl<const N: u8> Log<N> {
    /// Compile time assertion to check if `N` is correct.
    const VALID: () = assert!(N <= 4, "invalid LOG instruction");

    /// Create a new `LOG` instruction with the specified type.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::instruction::Log;
    /// let log: Log<4> = Log::new();
    /// ```
    ///
    /// This will fail to compile if the instruction is not correct.
    ///
    /// ```compile_fail
    /// # use eva_asm::instruction::Log;
    /// let log: Log<10> = Log::new(); // compile fail!
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        () = Self::VALID;
        Self { _private: () }
    }
}

impl<const N: u8> Default for Log<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u8> InstructionMeta for Log<N> {
    const OPCODE: OpCode = match N {
        0 => OpCode::LOG0,
        1 => OpCode::LOG1,
        2 => OpCode::LOG2,
        3 => OpCode::LOG3,
        4 => OpCode::LOG4,
        _ => unreachable!(),
    };
}
