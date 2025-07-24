//! Storage Operations.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Load word from storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SLoad;

impl KnownInstruction for SLoad {
    const MNEMONIC: Mnemonic = Mnemonic::SLOAD;
}

/// Save word to storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SStore;

impl KnownInstruction for SStore {
    const MNEMONIC: Mnemonic = Mnemonic::SSTORE;
}

/// Load word from transient storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct TLoad;

impl KnownInstruction for TLoad {
    const MNEMONIC: Mnemonic = Mnemonic::TLOAD;
}

/// Save word to transient storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct TStore;

impl KnownInstruction for TStore {
    const MNEMONIC: Mnemonic = Mnemonic::TSTORE;
}
