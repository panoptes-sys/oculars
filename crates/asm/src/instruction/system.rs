//! System and SHA3 operations.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Compute Keccak-256 hash.
pub struct Keccak256;

impl InstructionMeta for Keccak256 {
    const OPCODE: OpCode = OpCode::KECCAK256;
}

/// Create a new account with associated code.
pub struct Create;

impl InstructionMeta for Create {
    const OPCODE: OpCode = OpCode::CREATE;
}

/// Message-call into an account.
pub struct Call;

impl InstructionMeta for Call {
    const OPCODE: OpCode = OpCode::CALL;
}

/// Message-call into this account with alternative account’s code.
pub struct CallCode;

impl InstructionMeta for CallCode {
    const OPCODE: OpCode = OpCode::CALLCODE;
}

/// Halt execution returning output data.
pub struct Return;

impl InstructionMeta for Return {
    const OPCODE: OpCode = OpCode::RETURN;
}

/// Message-call into this account with an alternative account’s code, but persisting the current values for sender and value.
pub struct DelegateCall;

impl InstructionMeta for DelegateCall {
    const OPCODE: OpCode = OpCode::DELEGATECALL;
}

/// Create a new account with associated code at a predictable address.
pub struct Create2;

impl InstructionMeta for Create2 {
    const OPCODE: OpCode = OpCode::CREATE2;
}

/// Static message-call into an account.
pub struct StaticCall;

impl InstructionMeta for StaticCall {
    const OPCODE: OpCode = OpCode::STATICCALL;
}

/// Halt execution reverting state changes but returning data and remaining gas.
pub struct Revert;

impl InstructionMeta for Revert {
    const OPCODE: OpCode = OpCode::REVERT;
}

/// Designated invalid instruction.
pub struct Invalid;

impl InstructionMeta for Invalid {
    const OPCODE: OpCode = OpCode::INVALID;
}

/// Halt execution and register account for later deletion or send all Ether to address (post-Cancun).
pub struct SelfDestruct;

impl InstructionMeta for SelfDestruct {
    const OPCODE: OpCode = OpCode::SELFDESTRUCT;
}

/// An identified instruction.
/// The difference between this instruction and [`Invalid`] is that the [`Invalid`] instruction is explicitly
/// defined in the specification and this instruction is a catch-all instruction for any operation
/// code not defined in the specification. Otherwise they behave the exact same way.
pub struct Unknown(
    /// The unidentified operation code.
    pub u8,
);
