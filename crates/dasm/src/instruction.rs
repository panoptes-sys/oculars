//! Instruction assembly and disassembly.

use asm::instruction::{InstructionMeta, KnownInstruction, Push};
use thiserror::Error;

/// Extension trait over [`InstructionMeta`] allowing for instruction assembly and disassembly.
pub trait InstructionAssembly: Sized + InstructionMeta {
    /// The amount of bytes that is required to represent this instruction in bytecode.
    const BYTE_COUNT: usize;

    /// Assembles this instruction into bytecode.
    fn assemble(&self) -> [u8; Self::BYTE_COUNT];

    /// Disassembles this instruction from bytecode.
    ///
    /// # Errors
    /// Returns an error if the opcode (first byte) does not match the instruction's opcode.
    fn disassemble(bytes: [u8; Self::BYTE_COUNT]) -> Result<Self, UnexpectedOpcode>;
}

/// An error signifying that an unexpected opcode was encountered while disassembling an
/// instruction.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("unexpected opcode")]
pub struct UnexpectedOpcode;

impl<const N: usize> InstructionAssembly for Push<N> {
    const BYTE_COUNT: usize = N + 1;

    fn assemble(&self) -> [u8; Self::BYTE_COUNT] {
        let mut buf = [self.opcode().into_byte(); Self::BYTE_COUNT];
        buf[1..].copy_from_slice(self.immediate());
        buf
    }

    fn disassemble(bytes: [u8; Self::BYTE_COUNT]) -> Result<Self, UnexpectedOpcode> {
        let opcode = bytes[0];

        if opcode != Self::MNEMONIC as u8 {
            return Err(UnexpectedOpcode);
        }

        let mut immediate = [0u8; N];
        immediate.copy_from_slice(&bytes[1..]);

        Ok(Self::new(immediate))
    }
}

// impl InstructionAssembly for Unknown {
//     const BYTE_COUNT: usize = 1;
//
//     fn assemble(&self) -> [u8; Self::BYTE_COUNT] {
//         [self.0]
//     }
//
//     fn disassemble(bytes: [u8; Self::BYTE_COUNT]) -> Result<Self, UnexpectedOpcode> {
//         Ok(Self(bytes[0]))
//     }
// }

impl<I: KnownInstruction + Default> InstructionAssembly for I {
    default const BYTE_COUNT: usize = 1;

    default fn assemble(&self) -> [u8; Self::BYTE_COUNT] {
        [self.opcode().into_byte(); Self::BYTE_COUNT]
    }

    default fn disassemble(bytes: [u8; Self::BYTE_COUNT]) -> Result<Self, UnexpectedOpcode> {
        let opcode = bytes[0];

        if opcode != Self::MNEMONIC as u8 {
            return Err(UnexpectedOpcode);
        }

        Ok(Self::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_assembly() {
        assert_eq!(Push::<0>::new([]).assemble(), [Push::<0>::MNEMONIC as u8]);
        assert_eq!(
            Push::<4>::new([0xA; 4]).assemble(),
            [Push::<4>::MNEMONIC as u8, 0xA, 0xA, 0xA, 0xA]
        );

        let mut buf = [0xA; 33];
        buf[0] = Push::<32>::MNEMONIC as u8;
        assert_eq!(Push::<32>::new([0xA; 32]).assemble(), buf);
    }

    #[test]
    fn push_disassembly() {
        assert_eq!(
            Push::<0>::disassemble([Push::<0>::MNEMONIC as u8]).unwrap(),
            Push::<0>::new([])
        );

        assert_eq!(
            Push::<0>::disassemble([Push::<1>::MNEMONIC as u8]).unwrap_err(),
            UnexpectedOpcode
        );

        assert_eq!(
            Push::<4>::disassemble([Push::<4>::MNEMONIC as u8, 0xA, 0xA, 0xA, 0xA]).unwrap(),
            Push::<4>::new([0xA; 4]),
        );

        let mut buf = [0xA; 33];
        buf[0] = Push::<32>::MNEMONIC as u8;
        assert_eq!(Push::<32>::disassemble(buf).unwrap().immediate(), &buf[1..]);
    }
}
