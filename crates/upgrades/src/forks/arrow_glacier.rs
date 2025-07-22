//! Arrow Glacier network upgrade.

use chains::Mainnet;

use crate::{
    eip::macros::eip_set,
    eips::eip4345::Eip4345,
    execution::ExecutionUpgrade,
    forks::london::London,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Arrow Glacier network upgrade.
pub struct ArrowGlacier;

impl NetworkUpgrade for ArrowGlacier {}

impl ExecutionUpgrade for ArrowGlacier {
    type EipSet = eip_set!(London + Eip4345);
}

impl UpgradeActivation<Mainnet> for ArrowGlacier {
    fn block() -> u64 {
        13_773_000
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
        assert_eq!(ArrowGlacier::activation_block::<Mainnet>(), 13_773_000);
    }

    #[test]
    fn eip_support() {
        assert!(ArrowGlacier::includes::<Eip2>());

        assert!(ArrowGlacier::includes::<Eip4345>());
    }

    #[test]
    fn instruction_support() {
        assert!(ArrowGlacier::supports_instruction::<Add>());
    }
}
