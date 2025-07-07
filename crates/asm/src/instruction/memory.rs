//! Memory Operations.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Load word from memory.
pub struct MLoad;

impl InstructionMeta for MLoad {
    const OPCODE: OpCode = OpCode::MLOAD;
}

/// Save word to memory.
pub struct MStore;

impl InstructionMeta for MStore {
    const OPCODE: OpCode = OpCode::MSTORE;
}

/// Save byte to memory.
pub struct MStore8;

impl InstructionMeta for MStore8 {
    const OPCODE: OpCode = OpCode::MSTORE8;
}

/// Get the size of active memory in bytes.
pub struct MSize;

impl InstructionMeta for MSize {
    const OPCODE: OpCode = OpCode::MSIZE;
}

/// Copy memory areas.
pub struct MCopy;

impl InstructionMeta for MCopy {
    const OPCODE: OpCode = OpCode::MCOPY;
}
