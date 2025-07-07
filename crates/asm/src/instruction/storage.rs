//! Storage Operations.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Load word from storage.
pub struct SLoad;

impl InstructionMeta for SLoad {
    const OPCODE: OpCode = OpCode::SLOAD;
}

/// Save word to storage.
pub struct SStore;

impl InstructionMeta for SStore {
    const OPCODE: OpCode = OpCode::SSTORE;
}

/// Load word from transient storage.
pub struct TLoad;

impl InstructionMeta for TLoad {
    const OPCODE: OpCode = OpCode::TLOAD;
}

/// Save word to transient storage.
pub struct TStore;

impl InstructionMeta for TStore {
    const OPCODE: OpCode = OpCode::TSTORE;
}
