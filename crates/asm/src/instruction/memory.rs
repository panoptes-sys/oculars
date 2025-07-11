//! Memory Operations.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Load word from memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MLoad;

impl Instruction for MLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MLOAD)
    }
}

/// Save word to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MStore;

impl Instruction for MStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSTORE)
    }
}

/// Save byte to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MStore8;

impl Instruction for MStore8 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSTORE8)
    }
}

/// Get the size of active memory in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MSize;

impl Instruction for MSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MSIZE)
    }
}

/// Copy memory areas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MCopy;

impl Instruction for MCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MCOPY)
    }
}
