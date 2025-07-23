use asm::instruction::{InstructionMeta, Push};

/// Extension trait over [`InstructionMeta`] allowing assembling and disassembling instructions.
pub trait InstructionAssembly<const N: usize>: Sized + InstructionMeta {
    fn assemble(&self) -> [u8; N];
    fn disassemble(bytes: [u8; N]) -> Result<Self, InvalidOpcode>;
}

pub struct InvalidOpcode;

impl<const N: usize> InstructionAssembly<N> for Push<N> {
    fn assemble(&self) -> [u8; N] {
        let mut buf = [self.opcode().into_byte(); N];
        buf[1..].copy_from_slice(self.immediate());
        buf
    }

    fn disassemble(bytes: [u8; N]) -> Result<Self, InvalidOpcode> {
        let opcode = bytes[0];

        // if opcode != Push::<N>::opcode(&self)

        todo!()
    }
}
