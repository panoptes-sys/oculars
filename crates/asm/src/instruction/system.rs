//! System and SHA3 operations.

use super::KnownInstruction;
use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};
use derive_more::Display;

/// Compute Keccak-256 hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Keccak256;

impl KnownInstruction for Keccak256 {
    const MNEMONIC: Mnemonic = Mnemonic::KECCAK256;
}

/// Create a new account with associated code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Create;

impl KnownInstruction for Create {
    const MNEMONIC: Mnemonic = Mnemonic::CREATE;
}

/// Message-call into an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Call;

impl KnownInstruction for Call {
    const MNEMONIC: Mnemonic = Mnemonic::CALL;
}

/// Message-call into this account with alternative account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct CallCode;

impl KnownInstruction for CallCode {
    const MNEMONIC: Mnemonic = Mnemonic::CALLCODE;
}

/// Halt execution returning output data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Return;

impl KnownInstruction for Return {
    const MNEMONIC: Mnemonic = Mnemonic::RETURN;
}

/// Message-call into this account with an alternative account’s code, but persisting the current values for sender and value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct DelegateCall;

impl KnownInstruction for DelegateCall {
    const MNEMONIC: Mnemonic = Mnemonic::DELEGATECALL;
}

/// Create a new account with associated code at a predictable address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Create2;

impl KnownInstruction for Create2 {
    const MNEMONIC: Mnemonic = Mnemonic::CREATE2;
}

/// Static message-call into an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct StaticCall;

impl KnownInstruction for StaticCall {
    const MNEMONIC: Mnemonic = Mnemonic::STATICCALL;
}

/// Halt execution reverting state changes but returning data and remaining gas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Revert;

impl KnownInstruction for Revert {
    const MNEMONIC: Mnemonic = Mnemonic::REVERT;
}

/// Designated invalid instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Invalid;

impl KnownInstruction for Invalid {
    const MNEMONIC: Mnemonic = Mnemonic::INVALID;
}

/// Halt execution and register account for later deletion or send all Ether to address (post-Cancun).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SelfDestruct;

impl KnownInstruction for SelfDestruct {
    const MNEMONIC: Mnemonic = Mnemonic::SELFDESTRUCT;
}

/// An identified instruction.
/// The difference between this instruction and [`Invalid`] is that the [`Invalid`] instruction is explicit, Defaultly
/// defined in the specification and this instruction is a catch-all instruction for any operation
/// code not defined in the specification. Otherwise they behave the exact same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Unknown(
    /// The unidentified operation code.
    pub u8,
);

impl InstructionMeta for Unknown {
    fn opcode(&self) -> OpCode {
        OpCode::Unknown(self.0)
    }
}
