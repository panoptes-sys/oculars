//! Shanghai execution upgrade.

use crate::{
    eip_set,
    eips::{eip3651::Eip3651, eip3855::Eip3855, eip3860::Eip3860, eip4895::Eip4895},
    execution::{ExecutionUpgrade, paris::Paris},
};

/// Shanghai execution upgrade.
pub struct Shanghai;

impl ExecutionUpgrade for Shanghai {
    type EipSet = eip_set!(Paris + Eip3651, Eip3855, Eip3860, Eip4895);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::{Mnemonic, instruction::Add};

    #[test]
    fn instruction_support() {
        assert!(Shanghai::supports_instruction(&Add));
        assert!(Shanghai::supports_mnemonic(Mnemonic::PUSH0));
    }
}
