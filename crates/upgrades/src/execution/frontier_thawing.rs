//! Frontier Thawing network update.

use crate::execution::{ExecutionUpgrade, frontier::Frontier};

/// Frontier Thawing network update.
pub struct FrontierThawing;

impl ExecutionUpgrade for FrontierThawing {
    type EipSet = <Frontier as ExecutionUpgrade>::EipSet;
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{DelegateCall, Invalid, Stop};

    #[test]
    fn instruction_support() {
        assert!(Frontier::supports_instruction(&Stop));
        assert!(Frontier::supports_instruction(&Invalid));
        assert!(!Frontier::supports_instruction(&DelegateCall));
    }
}
