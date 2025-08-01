//! EVM instruction definitions.

mod dup;
mod log;
mod push;
mod swap;
mod unknown;

pub use crate::defs::instruction::*;
use crate::{defs::instruction::macros::match_instruction, AssemblyInstruction, OpCode};
pub use dup::Dup;
pub use log::Log;
pub use push::Push;
pub use swap::Swap;
pub use unknown::Unknown;

impl AssemblyInstruction for Instruction {
    fn opcode(&self) -> OpCode {
        match_instruction!(self, AssemblyInstruction::opcode)
    }

    fn immediate_size(&self) -> u8 {
        match_instruction!(self, AssemblyInstruction::immediate_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_fmt_is_sane() {
        assert_eq!(format!("{Gas:?}"), "Gas");
        assert_eq!(format!("{Gas}"), "GAS");
        assert_eq!(format!("{Gas:x}"), "5a");
        assert_eq!(format!("{Gas:X}"), "5A");
        assert_eq!(format!("{Gas:b}"), "1011010");
        assert_eq!(format!("{Gas:o}"), "132");
    }
}
