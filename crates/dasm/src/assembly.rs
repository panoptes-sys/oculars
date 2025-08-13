//! EVM assembly.

use asm::Instruction;

/// An instruction with a specified position in the bytecode.
pub struct PositionedInstruction {
    /// Position of this instruction in the bytecode.
    pub position: usize,

    /// The instruction at this position in the bytecode.
    pub instruction: Instruction,
}

impl PositionedInstruction {
    pub fn new(instruction: Instruction, position: usize) -> Self {
        Self {
            position,
            instruction,
        }
    }
}

/// Disassembled EVM bytecode.
pub struct Assembly {
    instructions: Vec<PositionedInstruction>,
}

impl Assembly {
    pub fn new(instructions: Vec<PositionedInstruction>) -> Self {
        Self { instructions }
    }

    pub fn instructions(&self) -> &[PositionedInstruction] {
        &self.instructions
    }
}
