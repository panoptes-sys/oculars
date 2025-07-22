//! Gray Glacier network upgrade.

use chains::Mainnet;

use crate::{
    eip::macros::eip_set,
    eips::eip5133::Eip5133,
    execution::ExecutionUpgrade,
    forks::arrow_glacier::ArrowGlacier,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Gray Glacier network upgrade.
pub struct GrayGlacier;

impl NetworkUpgrade for GrayGlacier {}

impl ExecutionUpgrade for GrayGlacier {
    type EipSet = eip_set!(ArrowGlacier + Eip5133);
}

impl UpgradeActivation<Mainnet> for GrayGlacier {
    fn block() -> u64 {
        15_050_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::Add;
    use chains::Mainnet;

    #[test]
    fn activation() {
        assert_eq!(GrayGlacier::activation_block::<Mainnet>(), 15_050_000);
    }

    #[test]
    fn eip_support() {
        assert!(GrayGlacier::includes::<Eip2>());

        assert!(GrayGlacier::includes::<Eip5133>());
    }

    #[test]
    fn instruction_support() {
        assert!(GrayGlacier::supports_instruction::<Add>());
    }
}
