//! Flow Operations.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Alter the program counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Jump;

impl KnownInstruction for Jump {
    const MNEMONIC: Mnemonic = Mnemonic::JUMP;
}

/// Conditionally alter the program counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct JumpI;

impl KnownInstruction for JumpI {
    const MNEMONIC: Mnemonic = Mnemonic::JUMPI;
}

/// Get the value of the program counter prior to the increment corresponding to this instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Pc;

impl KnownInstruction for Pc {
    const MNEMONIC: Mnemonic = Mnemonic::PC;
}

/// Get the amount of available gas, including the corresponding reduction for the cost of this instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Gas;

impl KnownInstruction for Gas {
    const MNEMONIC: Mnemonic = Mnemonic::GAS;
}

/// Mark a valid destination for jumps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct JumpDest;

impl KnownInstruction for JumpDest {
    const MNEMONIC: Mnemonic = Mnemonic::JUMPDEST;
}
