//! Flow Operations.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Alter the program counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Jump;

impl Instruction for Jump {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::JUMP)
    }
}

/// Conditionally alter the program counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct JumpI;

impl Instruction for JumpI {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::JUMPI)
    }
}

/// Get the value of the program counter prior to the increment corresponding to this instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Pc;

impl Instruction for Pc {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::PC)
    }
}

/// Get the amount of available gas, including the corresponding reduction for the cost of this instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Gas;

impl Instruction for Gas {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GAS)
    }
}

/// Mark a valid destination for jumps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct JumpDest;

impl Instruction for JumpDest {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::JUMPDEST)
    }
}
