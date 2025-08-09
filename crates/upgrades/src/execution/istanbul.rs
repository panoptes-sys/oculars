//! Istanbul network upgrade.

use crate::{
    eip_set,
    eips::{
        eip152::Eip152, eip1108::Eip1108, eip1344::Eip1344, eip1884::Eip1884, eip2028::Eip2028,
        eip2200::Eip2200,
    },
    execution::{ExecutionUpgrade, petersburg::Petersburg},
};

/// Istanbul network upgrade.
pub struct Istanbul;

impl ExecutionUpgrade for Istanbul {
    type EipSet = eip_set!(
        Petersburg + Eip152,
        Eip1108,
        Eip1344,
        Eip1884,
        Eip2028,
        Eip2200
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, ChainId, SelfBalance};

    #[test]
    fn instruction_support() {
        assert!(Istanbul::supports_instruction(&Add));

        assert!(Istanbul::supports_instruction(&ChainId));
        assert!(Istanbul::supports_instruction(&SelfBalance));
    }
}
