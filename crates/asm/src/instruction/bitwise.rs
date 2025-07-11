//! Comparison & Bitwise Logic Operations.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Less-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Lt;

impl Instruction for Lt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::LT)
    }
}

/// Greater-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Gt;

impl Instruction for Gt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GT)
    }
}

/// Signed less-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SLt;

impl Instruction for SLt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SLT)
    }
}

/// Signed greater-than comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SGt;

impl Instruction for SGt {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SGT)
    }
}

/// Equality comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Eq;

impl Instruction for Eq {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EQ)
    }
}

/// Is-zero comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct IsZero;

impl Instruction for IsZero {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ISZERO)
    }
}

/// Bitwise AND operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct And;

impl Instruction for And {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::AND)
    }
}

/// Bitwise OR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Or;

impl Instruction for Or {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::OR)
    }
}

/// Bitwise XOR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Xor;

impl Instruction for Xor {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::XOR)
    }
}

/// Bitwise NOT operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Not;

impl Instruction for Not {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::NOT)
    }
}

/// Retrieve single byte from word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Byte;

impl Instruction for Byte {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BYTE)
    }
}

/// Left shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Shl;

impl Instruction for Shl {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SHL)
    }
}

/// Logical right shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Shr;

impl Instruction for Shr {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SHR)
    }
}

/// Arithmetic (signed) right shift operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Sar;

impl Instruction for Sar {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SAR)
    }
}
