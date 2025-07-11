//! EVM instruction set.

mod arithmetic;
mod bitwise;
mod block;
mod environment;
mod flow;
mod logging;
mod memory;
mod stack;
mod storage;
mod system;

pub use arithmetic::*;
pub use bitwise::*;
pub use block::*;
pub use environment::*;
pub use flow::*;
pub use logging::*;
pub use memory::*;
pub use stack::*;
pub use storage::*;
pub use system::*;

use crate::opcode::OpCode;

/// General instruction information.
pub trait InstructionMeta {
    /// Return the operation code associated with this instruction.
    fn opcode(&self) -> OpCode;

    /// Returns a value signifying whether this instruction is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::instruction::{Push, Gas, InstructionMeta};
    /// assert_eq!(Push::new([0; 10]).is_push(), true);
    /// assert_eq!(Gas.is_push(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_push(&self) -> bool {
        self.opcode().is_push()
    }

    /// Returns a value signifying whether this instruction is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::instruction::{Dup, Gas, InstructionMeta};
    /// assert_eq!(Dup::<10>::new().is_dup(), true);
    /// assert_eq!(Gas.is_dup(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_dup(&self) -> bool {
        self.opcode().is_dup()
    }

    /// Returns a value signifying whether this instruction is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::instruction::{Swap, Gas, InstructionMeta};
    /// assert_eq!(Swap::<10>::new().is_swap(), true);
    /// assert_eq!(Gas.is_swap(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_swap(&self) -> bool {
        self.opcode().is_swap()
    }

    /// Returns a value signifying whether this instruction is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::instruction::{Log, Gas, InstructionMeta};
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
    /// # use eva_asm::instruction::{Return, Unknown, Gas, InstructionMeta};
    /// assert_eq!(Return.is_terminator(), true);
    /// assert_eq!(Unknown(0xF).is_terminator(), true);
    /// assert_eq!(Gas.is_terminator(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_terminator(&self) -> bool {
        self.opcode().is_terminator()
    }
}
