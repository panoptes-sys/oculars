//! Memory Operations.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Load word from memory.
pub struct MLoad;

impl InstructionMeta for MLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MLOAD)
    }
}

/// Save word to memory.
pub struct MStore;

impl InstructionMeta for MStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSTORE)
    }
}

/// Save byte to memory.
pub struct MStore8;

impl InstructionMeta for MStore8 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSTORE8)
    }
}

/// Get the size of active memory in bytes.
pub struct MSize;

impl InstructionMeta for MSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSIZE)
    }
}

/// Copy memory areas.
pub struct MCopy;

impl InstructionMeta for MCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MCOPY)
    }
}
