//! Byzantium network upgrade.

use crate::{
    eip_set,
    eips::{
        eip100::Eip100, eip140::Eip140, eip196::Eip196, eip197::Eip197, eip198::Eip198,
        eip211::Eip211, eip214::Eip214,
    },
    execution::{ExecutionUpgrade, spurious_dragon::SpuriousDragon},
};

/// Byzantium (Metropolis/Byzantium, Metropolis part 1) network upgrade.
pub struct Byzantium;

impl ExecutionUpgrade for Byzantium {
    type EipSet = eip_set!(
        SpuriousDragon + Eip100,
        Eip140,
        Eip196,
        Eip197,
        Eip198,
        Eip211,
        Eip214
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, DelegateCall, ReturnDataCopy, ReturnDataSize, Revert, StaticCall};

    #[test]
    fn instruction_support() {
        assert!(Byzantium::supports_instruction(&Add));
        assert!(Byzantium::supports_instruction(&DelegateCall));

        assert!(Byzantium::supports_instruction(&Revert));
        assert!(Byzantium::supports_instruction(&ReturnDataSize));
        assert!(Byzantium::supports_instruction(&ReturnDataCopy));
        assert!(Byzantium::supports_instruction(&StaticCall));
    }
}
