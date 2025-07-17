//! Ethereum Execution layer upgrades.

use asm::instruction::InstructionMeta;
use eips::{eip7::SupportsInstruction, Eip, IncludesEip};

/// Ethereum execution layer upgrade.
pub trait ExecutionUpgrade {
    /// Returns whether an EIP is included in an execution upgrade.
    #[must_use]
    #[inline]
    fn includes<E: Eip>() -> bool
    where
        Self: IncludesEip<E>,
    {
        <Self as IncludesEip<E>>::includes_eip()
    }
}

impl<I: InstructionMeta, EU: ExecutionUpgrade, E: Eip> SupportsInstruction<I> for EU
where
    EU: IncludesEip<E>,
    E: SupportsInstruction<I>,
{
    fn supports_instruction() -> bool {
        todo!()
    }
}

// crates/forks/src/execution.rs|19 col 26-28 error| the type parameter `EU` is not constrained by the impl trait, self type, or predicates unconstrained type parameter
// crates/forks/src/execution.rs|19 col 48-49 error| type parameter `E` must be used as the type parameter for some local type (e.g., `MyStruct<E>`) implementing a foreign trait is only possible if at least one of the types for which it is implemented is local only traits defined in the current crate can be implemented for a type parameter
