//! EVM instruction definitions.

mod dup;
mod log;
mod push;
mod swap;
mod unknown;

use crate::{
    AssemblyInstruction, Mnemonic, OpCode,
    assembly::DisassemblyError,
    defs::instruction::macros::{disassemble_instruction, match_instruction},
};

pub use crate::defs::instruction::*;
pub use dup::Dup;
pub use log::Log;
pub use push::Push;
pub use swap::Swap;
pub use unknown::Unknown;

impl AssemblyInstruction for Instruction {
    fn opcode(&self) -> OpCode {
        match_instruction!(self, AssemblyInstruction::opcode)
    }

    fn mnemonic(&self) -> Option<Mnemonic> {
        match_instruction!(self, AssemblyInstruction::mnemonic)
    }

    fn immediate_size(&self) -> u8 {
        match_instruction!(self, AssemblyInstruction::immediate_size)
    }

    fn disassemble(bytes: &[u8]) -> Result<Self, DisassemblyError> {
        disassemble_instruction!(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mnemonic;

    #[test]
    fn instruction_fmt_is_sane() {
        assert_eq!(format!("{Gas:?}"), "Gas");
        assert_eq!(format!("{Gas}"), "GAS");
        assert_eq!(format!("{Gas:x}"), "5a");
        assert_eq!(format!("{Gas:X}"), "5A");
        assert_eq!(format!("{Gas:b}"), "1011010");
        assert_eq!(format!("{Gas:o}"), "132");
    }

    #[test]
    fn it_disassembles_instructions() {
        assert_eq!(
            Instruction::disassemble(&[Mnemonic::GAS as u8]).unwrap(),
            Instruction::Gas(Gas)
        );

        assert_eq!(
            Instruction::disassemble(&[Mnemonic::PUSH2 as u8, 0xA, 0xB]).unwrap(),
            Instruction::Push2(Push::new([0xA, 0xB]))
        );

        assert_eq!(
            Instruction::disassemble(&[0xF]).unwrap(),
            Instruction::Unknown(Unknown::new(0xF))
        );

        Instruction::disassemble(&[]).unwrap_err();
        Instruction::disassemble(&[Mnemonic::PUSH2 as u8]).unwrap_err();
        Instruction::disassemble(&[Mnemonic::PUSH2 as u8, 0xA]).unwrap_err();
    }
}
