//! Storage Operations.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Load word from storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SLoad;

impl Instruction for SLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SLOAD)
    }
}

/// Save word to storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SStore;

impl Instruction for SStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SSTORE)
    }
}

/// Load word from transient storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct TLoad;

impl Instruction for TLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TLOAD)
    }
}

/// Save word to transient storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct TStore;

impl Instruction for TStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TSTORE)
    }
}
