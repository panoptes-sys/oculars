//! Paris execution upgrade.

use crate::{
    eip_set,
    eips::{eip3675::Eip3675, eip4399::Eip4399},
    execution::{ExecutionUpgrade, gray_glacier::GrayGlacier},
};

/// Paris execution upgrade.
pub struct Paris;

impl ExecutionUpgrade for Paris {
    type EipSet = eip_set!(GrayGlacier + Eip3675, Eip4399);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::Add;

    #[test]
    fn instruction_support() {
        assert!(Paris::supports_instruction(&Add));
    }
}
