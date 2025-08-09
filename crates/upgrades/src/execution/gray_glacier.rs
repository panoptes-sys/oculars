//! Gray Glacier network upgrade.

use crate::{
    eip_set,
    eips::eip5133::Eip5133,
    execution::{ExecutionUpgrade, arrow_glacier::ArrowGlacier},
};

/// Gray Glacier network upgrade.
pub struct GrayGlacier;

impl ExecutionUpgrade for GrayGlacier {
    type EipSet = eip_set!(ArrowGlacier + Eip5133);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::Add;

    #[test]
    fn instruction_support() {
        assert!(GrayGlacier::supports_instruction(&Add));
    }
}
