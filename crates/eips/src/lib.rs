//! Ethereum Improvement Proposals.

pub mod eip100;
pub mod eip140;
pub mod eip150;
pub mod eip155;
pub mod eip160;
pub mod eip161;
pub mod eip170;
pub mod eip196;
pub mod eip197;
pub mod eip198;
pub mod eip2;
pub mod eip211;
pub mod eip214;
pub mod eip649;
pub mod eip658;
pub mod eip7;

/// An Ethereum Improvement Proposal.
pub trait Eip {
    /// EIP number.
    ///
    /// # Example
    /// ```
    /// # use oculars_eips::{Eip, eip7::Eip7};
    /// assert_eq!(Eip7::NUMBER, 7);
    /// ```
    const NUMBER: u32;

    /// Return the EIP's number.
    ///
    /// # Example
    /// ```
    /// # use oculars_eips::{Eip, eip7::Eip7};
    /// assert_eq!(Eip7.number(), 7);
    /// ```
    fn number(&self) -> u32 {
        Self::NUMBER
    }
}

/// Trait that allows forks to specify which EIPs they support.
pub trait IncludesEip<E: Eip> {
    /// Returns whether an EIP is included in this fork.
    #[must_use]
    fn includes_eip() -> bool;
}
