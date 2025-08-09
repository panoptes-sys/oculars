//! Berlin network upgrade.

use crate::{
    eip_set,
    eips::{eip2565::Eip2565, eip2718::Eip2718, eip2929::Eip2929, eip2930::Eip2930},
    execution::{ExecutionUpgrade, muir_glacier::MuirGlacier},
};

/// Berlin network upgrade.
pub struct Berlin;

impl ExecutionUpgrade for Berlin {
    type EipSet = eip_set!(MuirGlacier + Eip2565, Eip2929, Eip2718, Eip2930);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::Add;

    #[test]
    fn instruction_support() {
        assert!(Berlin::supports_instruction(&Add));
    }
}
