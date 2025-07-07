//! Flow Operations.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Alter the program counter.
pub struct Jump;

impl InstructionMeta for Jump {
    const OPCODE: OpCode = OpCode::JUMP;
}

/// Conditionally alter the program counter.
pub struct JumpI;

impl InstructionMeta for JumpI {
    const OPCODE: OpCode = OpCode::JUMPI;
}

/// Get the value of the program counter prior to the increment corresponding to this instruction.
pub struct Pc;

impl InstructionMeta for Pc {
    const OPCODE: OpCode = OpCode::PC;
}

/// Get the amount of available gas, including the corresponding reduction for the cost of this instruction.
pub struct Gas;

impl InstructionMeta for Gas {
    const OPCODE: OpCode = OpCode::GAS;
}

/// Mark a valid destination for jumps.
pub struct JumpDest;

impl InstructionMeta for JumpDest {
    const OPCODE: OpCode = OpCode::JUMPDEST;
}
