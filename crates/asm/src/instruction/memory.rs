//! Memory Operations.

use derive_more::Display;

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Load word from memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MLoad;

impl InstructionMeta for MLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MLOAD)
    }
}

/// Save word to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MStore;

impl InstructionMeta for MStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSTORE)
    }
}

/// Save byte to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MStore8;

impl InstructionMeta for MStore8 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSTORE8)
    }
}

/// Get the size of active memory in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MSize;

impl InstructionMeta for MSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSIZE)
    }
}

/// Copy memory areas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MCopy;

impl InstructionMeta for MCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MCOPY)
    }
}
