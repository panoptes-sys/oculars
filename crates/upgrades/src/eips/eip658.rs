//! EIP-658: Embedding transaction status code in receipts.
//!
//! ## Abstract
//! This EIP replaces the intermediate state root field of the receipt with a status code indicating if the top-level call succeeded or failed.
//!
//! ## Motivation
//! With the introduction of the REVERT opcode in EIP140, it is no longer possible for users to assume that a transaction failed iff it consumed all gas. As a result, there is no clear mechanism for callers to determine whether a transaction succeeded and the state changes contained in it were applied.
//!
//! Full nodes can provide RPCs to get a transaction return status and value by replaying the transaction, but fast nodes can only do this for nodes after their pivot point, and light nodes cannot do this at all, making a non-consensus solution impractical.
//!
//! Instead, we propose to replace the intermediate state root, already obsoleted by EIP98, with the return status (1 for success, 0 for failure). This both allows callers to determine success status, and remedies the previous omission of return data from the receipt.
//!
//! ## Specification
//! For blocks where block.number >= `BYZANTIUM_FORK_BLKNUM`, the intermediate state root is replaced by a status code, 0 indicating failure (due to any operation that can cause the transaction or top-level call to revert) and 1 indicating success.
//!
//! ## Rationale
//! This constitutes a minimal possible change that permits fetching the success/failure state of transactions, preserving existing capabilities with minimum disruption or additional work for Metropolis.
//!
//! Nick Johnson <nick@ethereum.org>, "EIP-658: Embedding transaction status code in receipts," Ethereum Improvement Proposals, no. 658, June 2017. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-658>.

use crate::eip::Eip;

/// EIP-658: Embedding transaction status code in receipts.
pub struct Eip658;

impl Eip for Eip658 {
    const NUMBER: u32 = 658;
}
