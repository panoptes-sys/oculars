//! Stop and Arithmetic Operations.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Halts execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Stop;

impl Instruction for Stop {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::STOP)
    }
}

/// Addition operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Add;

impl Instruction for Add {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADD)
    }
}

/// Multiplication operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Mul;

impl Instruction for Mul {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MUL)
    }
}

/// Subtraction operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Sub;

impl Instruction for Sub {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SUB)
    }
}

/// Integer division operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Div;

impl Instruction for Div {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::DIV)
    }
}

/// Signed integer division operation (truncated).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SDiv;

impl Instruction for SDiv {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SDIV)
    }
}

/// Modulo remainder operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Mod;

impl Instruction for Mod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MOD)
    }
}

/// Signed modulo remainder operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SMod;

impl Instruction for SMod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SMOD)
    }
}

/// Modulo addition operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct AddMod;

impl Instruction for AddMod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADDMOD)
    }
}

/// Modulo multiplication operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct MulMod;

impl Instruction for MulMod {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::MULMOD)
    }
}

/// Exponential operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Exp;

impl Instruction for Exp {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXP)
    }
}

/// Extend length of two’s complement signed integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SignExtend;

impl Instruction for SignExtend {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SIGNEXTEND)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_fmt() {
        assert_eq!(format!("{Stop}"), String::from("STOP"));
    }
}
