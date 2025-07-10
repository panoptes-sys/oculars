//! Stop and Arithmetic Operations.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Halts execution.
pub struct Stop;

impl InstructionMeta for Stop {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::STOP)
    }
}

/// Addition operation.
pub struct Add;

impl InstructionMeta for Add {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADD)
    }
}

/// Multiplication operation.
pub struct Mul;

impl InstructionMeta for Mul {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MUL)
    }
}

/// Subtraction operation.
pub struct Sub;

impl InstructionMeta for Sub {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SUB)
    }
}

/// Integer division operation.
pub struct Div;

impl InstructionMeta for Div {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::DIV)
    }
}

/// Signed integer division operation (truncated).
pub struct SDiv;

impl InstructionMeta for SDiv {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SDIV)
    }
}

/// Modulo remainder operation.
pub struct Mod;

impl InstructionMeta for Mod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MOD)
    }
}

/// Signed modulo remainder operation.
pub struct SMod;

impl InstructionMeta for SMod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SMOD)
    }
}

/// Modulo addition operation.
pub struct AddMod;

impl InstructionMeta for AddMod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADDMOD)
    }
}

/// Modulo multiplication operation.
pub struct MulMod;

impl InstructionMeta for MulMod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MULMOD)
    }
}

/// Exponential operation.
pub struct Exp;

impl InstructionMeta for Exp {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXP)
    }
}

/// Extend length of two’s complement signed integer.
pub struct SignExtend;

impl InstructionMeta for SignExtend {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SIGNEXTEND)
    }
}
