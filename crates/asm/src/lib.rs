//! EVM instructions, opcodes and mnemonics.

#![deny(unsafe_code)]

pub mod assembly;
mod defs;
mod fmt;
pub mod instruction;
mod mnemonic;
mod opcode;

pub use assembly::AssemblyInstruction;
pub use instruction::Instruction;
pub use mnemonic::Mnemonic;
pub use opcode::OpCode;
