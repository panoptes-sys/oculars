//! Comparison & Bitwise Logic Operations.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Less-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Lt;

impl KnownInstruction for Lt {
    const MNEMONIC: Mnemonic = Mnemonic::LT;
}

/// Greater-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Gt;

impl KnownInstruction for Gt {
    const MNEMONIC: Mnemonic = Mnemonic::GT;
}

/// Signed less-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SLt;

impl KnownInstruction for SLt {
    const MNEMONIC: Mnemonic = Mnemonic::SLT;
}

/// Signed greater-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SGt;

impl KnownInstruction for SGt {
    const MNEMONIC: Mnemonic = Mnemonic::SGT;
}

/// Equality comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Eq;

impl KnownInstruction for Eq {
    const MNEMONIC: Mnemonic = Mnemonic::EQ;
}

/// Is-zero comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct IsZero;

impl KnownInstruction for IsZero {
    const MNEMONIC: Mnemonic = Mnemonic::ISZERO;
}

/// Bitwise AND operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct And;

impl KnownInstruction for And {
    const MNEMONIC: Mnemonic = Mnemonic::AND;
}

/// Bitwise OR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Or;

impl KnownInstruction for Or {
    const MNEMONIC: Mnemonic = Mnemonic::OR;
}

/// Bitwise XOR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Xor;

impl KnownInstruction for Xor {
    const MNEMONIC: Mnemonic = Mnemonic::XOR;
}

/// Bitwise NOT operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Not;

impl KnownInstruction for Not {
    const MNEMONIC: Mnemonic = Mnemonic::NOT;
}

/// Retrieve single byte from word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Byte;

impl KnownInstruction for Byte {
    const MNEMONIC: Mnemonic = Mnemonic::BYTE;
}

/// Left shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Shl;

impl KnownInstruction for Shl {
    const MNEMONIC: Mnemonic = Mnemonic::SHL;
}

/// Logical right shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Shr;

impl KnownInstruction for Shr {
    const MNEMONIC: Mnemonic = Mnemonic::SHR;
}

/// Arithmetic (signed) right shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Sar;

impl KnownInstruction for Sar {
    const MNEMONIC: Mnemonic = Mnemonic::SAR;
}
