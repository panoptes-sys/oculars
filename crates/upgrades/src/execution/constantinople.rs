//! Constantinople network upgrade.

use crate::{
    eip_set,
    eips::{
        eip145::Eip145, eip1014::Eip1014, eip1052::Eip1052, eip1234::Eip1234, eip1283::Eip1283,
    },
    execution::{ExecutionUpgrade, byzantium::Byzantium},
};

/// Constantinople (Metropolis/Constantinople, Metropolis part 2) network upgrade.
pub struct Constantinople;

impl ExecutionUpgrade for Constantinople {
    type EipSet = eip_set!(Byzantium + Eip145, Eip1014, Eip1052, Eip1234, Eip1283);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, Create2, ExtCodeHash, Sar, Shl, Shr};

    #[test]
    fn instruction_support() {
        assert!(Constantinople::supports_instruction(&Add));

        assert!(Constantinople::supports_instruction(&Shl));
        assert!(Constantinople::supports_instruction(&Shr));
        assert!(Constantinople::supports_instruction(&Sar));
        assert!(Constantinople::supports_instruction(&Create2));
        assert!(Constantinople::supports_instruction(&ExtCodeHash));
    }
}
