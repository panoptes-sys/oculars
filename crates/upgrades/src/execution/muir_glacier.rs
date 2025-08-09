//! Muir Glacier network upgrade.

use crate::{
    eip_set,
    eips::eip2384::Eip2384,
    execution::{ExecutionUpgrade, istanbul::Istanbul},
};

/// Muir Glacier network upgrade.
pub struct MuirGlacier;

impl ExecutionUpgrade for MuirGlacier {
    type EipSet = eip_set!(Istanbul + Eip2384);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, ChainId, SelfBalance};

    #[test]
    fn instruction_support() {
        assert!(MuirGlacier::supports_instruction(&Add));

        assert!(MuirGlacier::supports_instruction(&ChainId));
        assert!(MuirGlacier::supports_instruction(&SelfBalance));
    }
}
