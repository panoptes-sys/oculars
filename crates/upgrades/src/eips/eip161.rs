//! EIP-161: State trie cleaning.
//!
//! ### Hard fork
//!
//! Spurious Dragon
//!
//! ### Parameters
//! - `FORK_BLKNUM`: 2,675,000
//! - `CHAIN_ID`: 1 (Mainnet)
//!
//! ### Specification
//!
//! a. Account creation transactions and the `CREATE` operation SHALL, prior to the execution of the initialisation code, **increment** the **nonce** over and above its normal starting value by **one** (for normal networks, this will be simply 1, however test-nets with non-zero default starting nonces will be different).
//!
//! b. Whereas `CALL` and `SELFDESTRUCT` would charge 25,000 gas when the destination is non-existent, now the charge SHALL **only** be levied if the operation transfers **more than zero value** and the destination account is _dead_.
//!
//! c. No account may _change state_ from non-existent to existent-but-_empty_. If an operation would do this, the account SHALL instead remain non-existent.
//!
//! d. _At the end of the transaction_, any account _touched_ by the execution of that transaction which is now _empty_ SHALL instead become non-existent (i.e. **deleted**).
//!
//! Where:
//!
//! An account is considered to be _touched_ when it is involved in any potentially _state-changing_ operation. This includes, but is not limited to, being the recipient of a **transfer of zero value**.
//!
//! An account is considered _empty_ when it has **no code** and **zero nonce** and **zero balance**.
//!
//! An account is considered _dead_ when either it is non-existent or it is _empty_.
//!
//! _At the end of the transaction_ is immediately following the execution of the suicide list, prior to the determination of the state trie root for receipt population.
//!
//! An account _changes state_ when:
//! - it is the target or refund of a `SELFDESTRUCT` operation for **zero or more** value;
//! - it is the source or destination of a `CALL` operation or message-call transaction transferring **zero or more** value;
//! - it is the source or creation of a `CREATE` operation or contract-creation transaction endowing **zero or more** value;
//! - as the block author ("miner") it is the recipient of block-rewards or transaction-fees of **zero or more** value.
//!
//! #### Notes
//!
//! In the present Ethereum protocol, it should be noted that very few state changes can ultimately result in accounts that are empty following the execution of the transaction. In fact there are only four contexts that current implementations need track:
//! - an empty account has zero value transferred to it through `CALL`;
//! - an empty account has zero value transferred to it through `SELFDESTRUCT`;
//! - an empty account has zero value transferred to it through a message-call transaction;
//! - an empty account has zero value transferred to it through a zero-gas-price fees transfer.
//!
//! ### Rationale
//!
//! Same as #158 except that several edge cases are avoided since we do not break invariants:
//! - ~~that an account can go from having code and storage to not having code or storage mid-way through the execution of a transaction;~~ [corrected]
//! - that a newly created account cannot be deleted prior to being deployed.
//!
//! `CREATE` avoids zero in the nonce to avoid any suggestion of the oddity of `CREATE`d accounts being reaped half-way through their creation.
//!
//! ### Addendum (2017-08-15)
//!
//! On 2016-11-24, a consensus bug occurred due to two implementations having different behavior in the case of state reverts.[3] The specification was amended to clarify that empty account deletions are reverted when the state is reverted.
//!
//! Gavin Wood (@gavofyork), "EIP-161: State trie clearing (invariant-preserving alternative)," Ethereum Improvement Proposals, no. 161, October 2016. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-161>.

use crate::eip::Eip;

/// EIP-161: State trie cleaning.
pub struct Eip161;

impl Eip for Eip161 {
    const NUMBER: u32 = 161;
}
