//! Storage Operations.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Load word from storage.
pub struct SLoad;

impl InstructionMeta for SLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SLOAD)
    }
}

/// Save word to storage.
pub struct SStore;

impl InstructionMeta for SStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SSTORE)
    }
}

/// Load word from transient storage.
pub struct TLoad;

impl InstructionMeta for TLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TLOAD)
    }
}

/// Save word to transient storage.
pub struct TStore;

impl InstructionMeta for TStore {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TSTORE)
    }
}
