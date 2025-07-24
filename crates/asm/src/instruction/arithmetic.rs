//! Stop and Arithmetic Operations.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Halts execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Stop;

impl KnownInstruction for Stop {
    const MNEMONIC: Mnemonic = Mnemonic::STOP;
}

/// Addition operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Add;

impl KnownInstruction for Add {
    const MNEMONIC: Mnemonic = Mnemonic::ADD;
}

/// Multiplication operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Mul;

impl KnownInstruction for Mul {
    const MNEMONIC: Mnemonic = Mnemonic::MUL;
}

/// Subtraction operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Sub;

impl KnownInstruction for Sub {
    const MNEMONIC: Mnemonic = Mnemonic::SUB;
}

/// Integer division operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Div;

impl KnownInstruction for Div {
    const MNEMONIC: Mnemonic = Mnemonic::DIV;
}

/// Signed integer division operation (truncated).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SDiv;

impl KnownInstruction for SDiv {
    const MNEMONIC: Mnemonic = Mnemonic::SDIV;
}

/// Modulo remainder operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Mod;

impl KnownInstruction for Mod {
    const MNEMONIC: Mnemonic = Mnemonic::MOD;
}

/// Signed modulo remainder operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SMod;

impl KnownInstruction for SMod {
    const MNEMONIC: Mnemonic = Mnemonic::SMOD;
}

/// Modulo addition operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct AddMod;

impl KnownInstruction for AddMod {
    const MNEMONIC: Mnemonic = Mnemonic::ADDMOD;
}

/// Modulo multiplication operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct MulMod;

impl KnownInstruction for MulMod {
    const MNEMONIC: Mnemonic = Mnemonic::MULMOD;
}

/// Exponential operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct Exp;

impl KnownInstruction for Exp {
    const MNEMONIC: Mnemonic = Mnemonic::EXP;
}

/// Extend length of two’s complement signed integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Default)]
#[display("{}", self.opcode())]
pub struct SignExtend;

impl KnownInstruction for SignExtend {
    const MNEMONIC: Mnemonic = Mnemonic::SIGNEXTEND;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_fmt() {
        assert_eq!(format!("{Stop}"), String::from("STOP"));
    }
}
