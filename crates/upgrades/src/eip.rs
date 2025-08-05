//! Ethereum Improvement Proposals.

use std::any::TypeId;

use asm::AssemblyInstruction;

/// An Ethereum Improvement Proposal.
pub trait Eip {
    /// EIP number.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{eips::eip7::Eip7, eip::{Eip}};
    /// assert_eq!(Eip7::NUMBER, 7);
    /// ```
    const NUMBER: u32;

    /// Return the EIP's number.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{eips::eip7::Eip7, eip::{Eip}};
    /// assert_eq!(Eip7.number(), 7);
    /// ```
    #[must_use]
    #[inline]
    fn number(&self) -> u32 {
        Self::NUMBER
    }

    /// Returns whether this EIP introduced a specific instruction.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{execution::ExecutionUpgrade, eip::Eip, eips::{eip2::Eip2, eip7::Eip7}};
    /// # use asm::instruction::DelegateCall;
    /// assert!(!Eip2::introduces_instruction::<DelegateCall>());
    /// assert!(Eip7::introduces_instruction::<DelegateCall>());
    /// ```
    #[must_use]
    #[inline]
    fn introduces_instruction<I>() -> bool
    where
        I: AssemblyInstruction,
        Self: IntroducesInstruction<I>,
    {
        Self::eip_introduces_instruction()
    }
}

/// Trait that indicates whether EIP introduces an instruction.
pub trait IntroducesInstruction<I: AssemblyInstruction> {
    /// Returns whether this EIP introduced an instruction.
    #[must_use]
    fn eip_introduces_instruction() -> bool;
}

// Make every [`Eip`] introduce no instructions by default.
impl<I: AssemblyInstruction, E: Eip> IntroducesInstruction<I> for E {
    default fn eip_introduces_instruction() -> bool {
        false
    }
}

/// A collection of [`Eip`]s.
pub trait EipSet {
    /// Returns whether this set contains a specific [`Eip`].
    fn includes_eip<E: Eip + 'static>() -> bool;

    /// Returns whether this set supports an instruction.
    fn supports_instruction<I: AssemblyInstruction>() -> bool;
}

// An empty tuple is considered an [`EipSet`].
impl EipSet for () {
    fn includes_eip<E: Eip + 'static>() -> bool {
        false
    }

    fn supports_instruction<I: AssemblyInstruction>() -> bool {
        false
    }
}

// A tuple of an [`Eip`] and a [`EipSet`] is considered an [`EipSet`].
impl<A: Eip + 'static, B: EipSet> EipSet for (A, B) {
    fn includes_eip<E: Eip + 'static>() -> bool {
        TypeId::of::<A>() == TypeId::of::<E>() || B::includes_eip::<E>()
    }

    fn supports_instruction<I: AssemblyInstruction>() -> bool {
        <A as IntroducesInstruction<I>>::eip_introduces_instruction()
            || B::supports_instruction::<I>()
    }
}

// Implement [`IntroducesInstruction`] for [`EipSet`]s.
impl<A, B, I> IntroducesInstruction<I> for (A, B)
where
    A: IntroducesInstruction<I>,
    B: IntroducesInstruction<I>,
    I: AssemblyInstruction,
{
    fn eip_introduces_instruction() -> bool {
        A::eip_introduces_instruction() || B::eip_introduces_instruction()
    }
}

// An empty [`EipSet`] introduces no instructions.
impl<I> IntroducesInstruction<I> for ()
where
    I: AssemblyInstruction,
{
    fn eip_introduces_instruction() -> bool {
        false
    }
}

/// EIP helper macros.
pub mod macros {
    /// Create an [`super::EipSet`] from a list of EIPs.
    /// Can extend an existing [`super::EipSet`] using the `eip_set!(OtherSet + Eip1, Eip2)` syntax.
    macro_rules! eip_set {
        ($a: ident) => {
            ($a, ())
        };
        ($a: ident, $($b: ident),+) => {
            ($a, eip_set!($($b),+))
        };
        ($upgrade: ident + $a: ident)=> {
            ($a, <$upgrade as $crate::execution::ExecutionUpgrade>::EipSet)
        };
        ($upgrade: ident + $a: ident, $($rest: ident),+) => {
            ($a, eip_set!($upgrade + $($rest),+))
        };
    }

    /// Specifies that this EIP introduces a new instruction.
    macro_rules! introduces_instructions {
        ($eip: ident, $($instruction: path),+) => {
            $(impl $crate::eip::IntroducesInstruction<$instruction> for $eip {
                fn eip_introduces_instruction() -> bool {
                    true
                }
            })+
        };
    }

    pub(crate) use eip_set;
    pub(crate) use introduces_instructions;
}
