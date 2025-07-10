//! System and SHA3 operations.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Compute Keccak-256 hash.
pub struct Keccak256;

impl InstructionMeta for Keccak256 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::KECCAK256)
    }
}

/// Create a new account with associated code.
pub struct Create;

impl InstructionMeta for Create {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CREATE)
    }
}

/// Message-call into an account.
pub struct Call;

impl InstructionMeta for Call {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALL)
    }
}

/// Message-call into this account with alternative account’s code.
pub struct CallCode;

impl InstructionMeta for CallCode {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLCODE)
    }
}

/// Halt execution returning output data.
pub struct Return;

impl InstructionMeta for Return {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURN)
    }
}

/// Message-call into this account with an alternative account’s code, but persisting the current values for sender and value.
pub struct DelegateCall;

impl InstructionMeta for DelegateCall {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::DELEGATECALL)
    }
}

/// Create a new account with associated code at a predictable address.
pub struct Create2;

impl InstructionMeta for Create2 {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CREATE2)
    }
}

/// Static message-call into an account.
pub struct StaticCall;

impl InstructionMeta for StaticCall {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::STATICCALL)
    }
}

/// Halt execution reverting state changes but returning data and remaining gas.
pub struct Revert;

impl InstructionMeta for Revert {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::REVERT)
    }
}

/// Designated invalid instruction.
pub struct Invalid;

impl InstructionMeta for Invalid {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::INVALID)
    }
}

/// Halt execution and register account for later deletion or send all Ether to address (post-Cancun).
pub struct SelfDestruct;

impl InstructionMeta for SelfDestruct {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SELFDESTRUCT)
    }
}

/// An identified instruction.
/// The difference between this instruction and [`Invalid`] is that the [`Invalid`] instruction is explicitly
/// defined in the specification and this instruction is a catch-all instruction for any operation
/// code not defined in the specification. Otherwise they behave the exact same way.
pub struct Unknown(
    /// The unidentified operation code.
    pub u8,
);

impl InstructionMeta for Unknown {
    fn opcode(&self) -> OpCode {
        OpCode::Unknown(self.0)
    }
}
