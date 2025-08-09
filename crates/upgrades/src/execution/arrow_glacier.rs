//! Arrow Glacier network upgrade.

use crate::{
    eip_set,
    eips::eip4345::Eip4345,
    execution::{ExecutionUpgrade, london::London},
};

/// Arrow Glacier network upgrade.
pub struct ArrowGlacier;

impl ExecutionUpgrade for ArrowGlacier {
    type EipSet = eip_set!(London + Eip4345);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::Add;

    #[test]
    fn instruction_support() {
        assert!(ArrowGlacier::supports_instruction(&Add));
    }
}
