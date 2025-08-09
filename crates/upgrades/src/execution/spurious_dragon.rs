//! Spurious Dragon network update.

use crate::{
    eip_set,
    eips::{eip155::Eip155, eip160::Eip160, eip161::Eip161, eip170::Eip170},
    execution::{ExecutionUpgrade, tangerine_whistle::TangerineWhistle},
};

/// Spurious Dragon network update.
pub struct SpuriousDragon;

impl ExecutionUpgrade for SpuriousDragon {
    type EipSet = eip_set!(TangerineWhistle + Eip155, Eip160, Eip161, Eip170);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, DelegateCall};

    #[test]
    fn instruction_support() {
        assert!(SpuriousDragon::supports_instruction(&Add));
        assert!(SpuriousDragon::supports_instruction(&DelegateCall));
    }
}
