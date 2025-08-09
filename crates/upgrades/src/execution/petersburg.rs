//! Petersburg network upgrade.

use crate::{
    eip_set,
    eips::{eip145::Eip145, eip1014::Eip1014, eip1052::Eip1052, eip1234::Eip1234},
    execution::{ExecutionUpgrade, byzantium::Byzantium},
};

/// Petersburg (St. Peretsfork, Peter's Fork, Constantinople Fix) network upgrade.
pub struct Petersburg;

impl ExecutionUpgrade for Petersburg {
    type EipSet = eip_set!(Byzantium + Eip145, Eip1014, Eip1052, Eip1234);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, Create2, ExtCodeHash, Sar, Shl, Shr};

    #[test]
    fn instruction_support() {
        assert!(Petersburg::supports_instruction(&Add));

        assert!(Petersburg::supports_instruction(&Shl));
        assert!(Petersburg::supports_instruction(&Shr));
        assert!(Petersburg::supports_instruction(&Sar));
        assert!(Petersburg::supports_instruction(&Create2));
        assert!(Petersburg::supports_instruction(&ExtCodeHash));
    }
}
