//! Storage Operations.

use derive_more::Display;

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Load word from storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SLoad;

impl InstructionMeta for SLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SLOAD)
    }
}

/// Save word to storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SStore;

impl InstructionMeta for SStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SSTORE)
    }
}

/// Load word from transient storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct TLoad;

impl InstructionMeta for TLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TLOAD)
    }
}

/// Save word to transient storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct TStore;

impl InstructionMeta for TStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TSTORE)
    }
}
