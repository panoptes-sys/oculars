//! Comparison & Bitwise Logic Operations.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Less-than comparison.
pub struct Lt;

impl InstructionMeta for Lt {
    const OPCODE: OpCode = OpCode::LT;
}

/// Greater-than comparison.
pub struct Gt;

impl InstructionMeta for Gt {
    const OPCODE: OpCode = OpCode::GT;
}

/// Signed less-than comparison.
pub struct SLt;

impl InstructionMeta for SLt {
    const OPCODE: OpCode = OpCode::SLT;
}

/// Signed greater-than comparison.
pub struct SGt;

impl InstructionMeta for SGt {
    const OPCODE: OpCode = OpCode::SGT;
}

/// Equality comparison.
pub struct Eq;

impl InstructionMeta for Eq {
    const OPCODE: OpCode = OpCode::EQ;
}

/// Is-zero comparison.
pub struct IsZero;

impl InstructionMeta for IsZero {
    const OPCODE: OpCode = OpCode::ISZERO;
}

/// Bitwise AND operation.
pub struct And;

impl InstructionMeta for And {
    const OPCODE: OpCode = OpCode::AND;
}

/// Bitwise OR operation.
pub struct Or;

impl InstructionMeta for Or {
    const OPCODE: OpCode = OpCode::OR;
}

/// Bitwise XOR operation.
pub struct Xor;

impl InstructionMeta for Xor {
    const OPCODE: OpCode = OpCode::XOR;
}

/// Bitwise NOT operation.
pub struct Not;

impl InstructionMeta for Not {
    const OPCODE: OpCode = OpCode::NOT;
}

/// Retrieve single byte from word.
pub struct Byte;

impl InstructionMeta for Byte {
    const OPCODE: OpCode = OpCode::BYTE;
}

/// Left shift operation.
pub struct Shl;

impl InstructionMeta for Shl {
    const OPCODE: OpCode = OpCode::SHL;
}

/// Logical right shift operation.
pub struct Shr;

impl InstructionMeta for Shr {
    const OPCODE: OpCode = OpCode::SHR;
}

/// Arithmetic (signed) right shift operation.
pub struct Sar;

impl InstructionMeta for Sar {
    const OPCODE: OpCode = OpCode::SAR;
}
