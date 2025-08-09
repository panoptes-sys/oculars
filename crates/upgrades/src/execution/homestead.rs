//! Homestead network update.

use crate::{
    eip_set,
    eips::{eip2::Eip2, eip7::Eip7, eip8::Eip8},
    execution::{ExecutionUpgrade, frontier_thawing::FrontierThawing},
};

/// Homestead network update.
pub struct Homestead;

impl ExecutionUpgrade for Homestead {
    type EipSet = eip_set!(FrontierThawing + Eip2, Eip7, Eip8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, DelegateCall};

    #[test]
    fn instruction_support() {
        assert!(Homestead::supports_instruction(&Add));
        assert!(Homestead::supports_instruction(&DelegateCall));
    }
}
