//! Comparison & Bitwise Logic Operations.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Less-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lt;

impl InstructionMeta for Lt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::LT)
    }
}

/// Greater-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gt;

impl InstructionMeta for Gt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GT)
    }
}

/// Signed less-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SLt;

impl InstructionMeta for SLt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SLT)
    }
}

/// Signed greater-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SGt;

impl InstructionMeta for SGt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SGT)
    }
}

/// Equality comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Eq;

impl InstructionMeta for Eq {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EQ)
    }
}

/// Is-zero comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsZero;

impl InstructionMeta for IsZero {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ISZERO)
    }
}

/// Bitwise AND operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct And;

impl InstructionMeta for And {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::AND)
    }
}

/// Bitwise OR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Or;

impl InstructionMeta for Or {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::OR)
    }
}

/// Bitwise XOR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Xor;

impl InstructionMeta for Xor {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::XOR)
    }
}

/// Bitwise NOT operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Not;

impl InstructionMeta for Not {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::NOT)
    }
}

/// Retrieve single byte from word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Byte;

impl InstructionMeta for Byte {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BYTE)
    }
}

/// Left shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shl;

impl InstructionMeta for Shl {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SHL)
    }
}

/// Logical right shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shr;

impl InstructionMeta for Shr {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SHR)
    }
}

/// Arithmetic (signed) right shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sar;

impl InstructionMeta for Sar {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SAR)
    }
}
