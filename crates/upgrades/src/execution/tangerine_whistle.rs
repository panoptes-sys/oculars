//! Tangerine Whistle network update.

use crate::{
    eips::eip150::Eip150,
    execution::{ExecutionUpgrade, homestead::Homestead},
};

/// Tangerine Whistle network update.
pub struct TangerineWhistle;

impl ExecutionUpgrade for TangerineWhistle {
    type EipSet = (Eip150, <Homestead as ExecutionUpgrade>::EipSet);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, DelegateCall};

    #[test]
    fn instruction_support() {
        assert!(TangerineWhistle::supports_instruction(&Add));
        assert!(TangerineWhistle::supports_instruction(&DelegateCall));
    }
}
