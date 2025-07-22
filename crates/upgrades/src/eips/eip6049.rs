//! EIP-6049: Deprecate SELFDESTRUCT.
//!
//! ## Abstract
//!
//! This EIP deprecates the `SELFDESTRUCT` opcode and warns against its use. A breaking change to this functionality is likely to come in the future.
//!
//! ## Motivation
//!
//! Discussions about how to change `SELFDESTRUCT` are ongoing. But there is a strong consensus that *something* will change.
//!
//! ## Specification
//!
//! Documentation of the `SELFDESTRUCT` opcode is updated to warn against its use and to note that a breaking change may be forthcoming.
//!
//! ## Rationale
//!
//! As time goes on, the cost of doing something increases, because any change to `SELFDESTRUCT` will be a breaking change.
//!
//! The Ethereum Blog and other official sources have not provided any warning to developers about a potential forthcoming change.
//!
//! ## Backwards Compatibility
//!
//! This EIP updates non-normative text in the Yellow Paper. No changes to clients is applicable.
//!
//! ## Security Considerations
//!
//! None.
//!
//! William Entriken (@fulldecent), "EIP-6049: Deprecate SELFDESTRUCT," Ethereum Improvement Proposals, no. 6049, November 2022. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-6049>.

use crate::eip::Eip;

/// EIP-6049: Deprecate SELFDESTRUCT.
pub struct Eip6049;

impl Eip for Eip6049 {
    const NUMBER: u32 = 6049;
}
