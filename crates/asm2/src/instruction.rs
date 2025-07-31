use crate::{opcode::OpCode, Mnemonic};

pub trait InstructionAssembly {
    const OPCODE: OpCode;

    const SIZE: usize = 1;

    fn size(&self) -> usize {
        Self::SIZE
    }
}

macro_rules! define_instructions {
    ($($name: ident, $mnemonic: ident),+) => {
        $(
            pub struct $name;

            impl InstructionAssembly for $name {
                const OPCODE: OpCode = OpCode::Known(Mnemonic::$mnemonic);
            }
        )+
    };
}

define_instructions!(Stop, STOP);
