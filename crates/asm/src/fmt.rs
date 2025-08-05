//! Formatting macros.

/// Implements formatting for `struct`s that implement `Into<u8>` by forwarding the formatting
/// implementation to [`u8`].
macro_rules! forward_byte_fmt {
    ($struct: ident, $($fmt: ident),+) => {
        $(
            impl std::fmt::$fmt for $struct {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::$fmt::fmt(&u8::from(*self), f)
                }
            }
        )+
    };
}

/// Forwards the formatting implementation of an instruction to its opcode.
macro_rules! forward_opcode_fmt {
    ($struct: ident, $($fmt: ident),+) => {
        $(
            impl std::fmt::$fmt for $struct {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::$fmt::fmt(&$crate::AssemblyInstruction::opcode(self), f)
                }
            }
        )+
    };
    (generic $struct: ident, $($fmt: ident),+) => {
        $(
            impl<const N: u8> std::fmt::$fmt for $struct<N> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::$fmt::fmt(&$crate::AssemblyInstruction::opcode(self), f)
                }
            }
        )+
    }
}

pub(crate) use forward_byte_fmt;
pub(crate) use forward_opcode_fmt;

#[cfg(test)]
mod tests {
    use crate::{AssemblyInstruction, Mnemonic, OpCode};

    #[test]
    fn byte_fmt_works() {
        #[derive(Copy, Clone)]
        struct Mock;

        impl From<Mock> for u8 {
            fn from(_: Mock) -> Self {
                10
            }
        }

        forward_byte_fmt!(Mock, LowerHex);

        assert_eq!(format!("{Mock:x}"), "a");
    }

    #[test]
    fn forward_opcode_fmt_works() {
        struct Mock;

        impl AssemblyInstruction for Mock {
            fn opcode(&self) -> OpCode {
                OpCode::Known(Mnemonic::GAS)
            }

            fn disassemble(_: &[u8]) -> Result<Self, crate::assembly::DisassemblyError> {
                todo!()
            }
        }

        forward_opcode_fmt!(Mock, Display, LowerHex, UpperHex, Binary, Octal);

        assert_eq!(format!("{Mock}"), "GAS");
        assert_eq!(format!("{Mock:x}"), "5a");
        assert_eq!(format!("{Mock:X}"), "5A");
        assert_eq!(format!("{Mock:b}"), "1011010");
        assert_eq!(format!("{Mock:o}"), "132");
    }
}
