//! Frontier network update.

use crate::{eip_set, eips::genesis::Genesis, execution::ExecutionUpgrade};

/// Frontier network update.
pub struct Frontier;

impl ExecutionUpgrade for Frontier {
    type EipSet = eip_set!(Genesis);
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
