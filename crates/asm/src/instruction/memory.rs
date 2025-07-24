//! Memory Operations.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Load word from memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct MLoad;

impl KnownInstruction for MLoad {
    const MNEMONIC: Mnemonic = Mnemonic::MLOAD;
}

/// Save word to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct MStore;

impl KnownInstruction for MStore {
    const MNEMONIC: Mnemonic = Mnemonic::MSTORE;
}

/// Save byte to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct MStore8;

impl KnownInstruction for MStore8 {
    const MNEMONIC: Mnemonic = Mnemonic::MSTORE8;
}

/// Get the size of active memory in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct MSize;

impl KnownInstruction for MSize {
    const MNEMONIC: Mnemonic = Mnemonic::MSIZE;
}

/// Copy memory areas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct MCopy;

impl KnownInstruction for MCopy {
    const MNEMONIC: Mnemonic = Mnemonic::MCOPY;
}
