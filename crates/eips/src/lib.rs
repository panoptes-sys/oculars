//! Ethereum Improvement Proposals.

pub mod eip150;
pub mod eip155;
pub mod eip160;
pub mod eip161;
pub mod eip170;
pub mod eip2;
pub mod eip7;

/// Marker trait to mark an EIP.
pub trait Eip {}

/// Trait that allows forks to specify which EIPs they support.
pub trait IncludesEip<E: Eip> {
    /// Returns whether an EIP is included in this fork.
    #[must_use]
    fn includes_eip() -> bool;
}
