//! EVM assembly.

use asm::instruction::Instruction;

/// An instruction with a specified position in the bytecode.
pub struct PositionedInstruction {
    /// Position of this instruction in the bytecode.
    pub position: usize,

    /// The instruction at this position in the bytecode.
    pub instruction: Instruction,
}

/// Disassembled EVM bytecode.
pub struct Assembly(/* Vec<PositionedInstruction> */);
