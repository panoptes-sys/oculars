//! Paris execution upgrade.

use crate::{
    eip::macros::eip_set,
    eips::{eip3675::Eip3675, eip4399::Eip4399},
    execution::ExecutionUpgrade,
    forks::gray_glacier::GrayGlacier,
};

/// Paris execution upgrade.
pub struct Paris;

impl ExecutionUpgrade for Paris {
    type EipSet = eip_set!(GrayGlacier + Eip3675, Eip4399);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::Add;

    #[test]
    fn eip_support() {
        assert!(Paris::includes::<Eip2>());

        assert!(Paris::includes::<Eip3675>());
        assert!(Paris::includes::<Eip4399>());
    }

    #[test]
    fn instruction_support() {
        assert!(Paris::supports_instruction::<Add>());
    }
}
