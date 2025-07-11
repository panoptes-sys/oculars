//! System and SHA3 operations.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Compute Keccak-256 hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Keccak256;

impl Instruction for Keccak256 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::KECCAK256)
    }
}

/// Create a new account with associated code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Create;

impl Instruction for Create {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CREATE)
    }
}

/// Message-call into an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Call;

impl Instruction for Call {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALL)
    }
}

/// Message-call into this account with alternative account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallCode;

impl Instruction for CallCode {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLCODE)
    }
}

/// Halt execution returning output data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Return;

impl Instruction for Return {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURN)
    }
}

/// Message-call into this account with an alternative account’s code, but persisting the current values for sender and value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct DelegateCall;

impl Instruction for DelegateCall {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::DELEGATECALL)
    }
}

/// Create a new account with associated code at a predictable address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Create2;

impl Instruction for Create2 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CREATE2)
    }
}

/// Static message-call into an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct StaticCall;

impl Instruction for StaticCall {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::STATICCALL)
    }
}

/// Halt execution reverting state changes but returning data and remaining gas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Revert;

impl Instruction for Revert {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::REVERT)
    }
}

/// Designated invalid instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Invalid;

impl Instruction for Invalid {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::INVALID)
    }
}

/// Halt execution and register account for later deletion or send all Ether to address (post-Cancun).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SelfDestruct;

impl Instruction for SelfDestruct {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SELFDESTRUCT)
    }
}

/// An identified instruction.
/// The difference between this instruction and [`Invalid`] is that the [`Invalid`] instruction is explicitly
/// defined in the specification and this instruction is a catch-all instruction for any operation
/// code not defined in the specification. Otherwise they behave the exact same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Unknown(
    /// The unidentified operation code.
    pub u8,
);

impl Instruction for Unknown {
    fn opcode(&self) -> OpCode {
        OpCode::Unknown(self.0)
    }
}
