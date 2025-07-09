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
    /// Operation code associated with this instruction.
    const OPCODE: OpCode;

    /// Return the operation code associated with this instruction.
    #[inline]
    fn opcode(&self) -> OpCode {
        Self::OPCODE
    }
}
