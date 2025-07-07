//! Stop and Arithmetic Operations.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Halts execution.
pub struct Stop;

impl InstructionMeta for Stop {
    const OPCODE: OpCode = OpCode::STOP;
}

/// Addition operation.
pub struct Add;

impl InstructionMeta for Add {
    const OPCODE: OpCode = OpCode::ADD;
}

/// Multiplication operation.
pub struct Mul;

impl InstructionMeta for Mul {
    const OPCODE: OpCode = OpCode::MUL;
}

/// Subtraction operation.
pub struct Sub;

impl InstructionMeta for Sub {
    const OPCODE: OpCode = OpCode::SUB;
}

/// Integer division operation.
pub struct Div;

impl InstructionMeta for Div {
    const OPCODE: OpCode = OpCode::DIV;
}

/// Signed integer division operation (truncated).
pub struct SDiv;

impl InstructionMeta for SDiv {
    const OPCODE: OpCode = OpCode::SDIV;
}

/// Modulo remainder operation.
pub struct Mod;

impl InstructionMeta for Mod {
    const OPCODE: OpCode = OpCode::MOD;
}

/// Signed modulo remainder operation.
pub struct SMod;

impl InstructionMeta for SMod {
    const OPCODE: OpCode = OpCode::SMOD;
}

/// Modulo addition operation.
pub struct AddMod;

impl InstructionMeta for AddMod {
    const OPCODE: OpCode = OpCode::ADDMOD;
}

/// Modulo multiplication operation.
pub struct MulMod;

impl InstructionMeta for MulMod {
    const OPCODE: OpCode = OpCode::MULMOD;
}

/// Exponential operation.
pub struct Exp;

impl InstructionMeta for Exp {
    const OPCODE: OpCode = OpCode::EXP;
}

/// Extend length of two’s complement signed integer.
pub struct SignExtend;

impl InstructionMeta for SignExtend {
    const OPCODE: OpCode = OpCode::SIGNEXTEND;
}
