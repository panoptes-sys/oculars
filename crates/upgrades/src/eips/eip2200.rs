//! EIP-220: Structured Definitions for Net Gas Metering.
//! ## Simple Summary
//!
//! This is an EIP that implements net gas metering. It's a combined
//! version of [EIP-1283] and [EIP-1706], with a structured definition so as
//! to make it interoperable with other gas changes such as [EIP-1884].
//!
//! ## Abstract
//!
//! This EIP provides a structured definition of net gas metering changes
//! for `SSTORE` opcode, enabling new usages for contract storage, and
//! reducing excessive gas costs where it doesn’t match how most
//! implementation works.
//!
//! This is a combination of [EIP-1283] and [EIP-1706].
//!
//! ## Motivation
//!
//! This EIP proposes a way for gas metering on `SSTORE`, using information
//! that is more universally available to most implementations, and
//! require as little change in implementation structures as possible.
//!
//! * Storage slot’s original value.
//! * Storage slot’s current value.
//! * Refund counter.
//!
//! Usages that benefits from this EIP’s gas reduction scheme includes:
//!
//! * Subsequent storage write operations within the same call frame. This
//!   includes reentry locks, same-contract multi-send, etc.
//! * Exchange storage information between sub call frame and parent call
//!   frame, where this information does not need to be persistent outside
//!   of a transaction. This includes sub-frame error codes and message
//!   passing, etc.
//!
//! The original definition of EIP-1283 created a danger of a new kind of
//! reentrancy attacks on existing contracts as Solidity by default grants
//! a "stipend" of 2300 gas to simple transfer calls. This danger is
//! easily mitigated if `SSTORE` is not allowed in low gasleft state,
//! without breaking the backward compatibility and the original intention
//! of EIP-1283.
//!
//! This EIP also replaces the original EIP-1283 value definitions of gas
//! by parameters, so that it's more structured, and easier to define
//! changes in the future.
//!
//! ## Specification
//!
//! Define variables `SLOAD_GAS`, `SSTORE_SET_GAS`, `SSTORE_RESET_GAS` and
//! `SSTORE_CLEARS_SCHEDULE`. The old and new values for those variables
//! are:
//!
//! * `SLOAD_GAS`: changed from `200` to `800`.
//! * `SSTORE_SET_GAS`: `20000`, not changed.
//! * `SSTORE_RESET_GAS`: `5000`, not changed.
//! * `SSTORE_CLEARS_SCHEDULE`: `15000`, not changed.
//!
//! Change the definition of EIP-1283 using those variables. The new
//! specification, combining EIP-1283 and EIP-1706, will look like
//! below. The terms *original value*, *current value* and *new value* are
//! defined in EIP-1283.
//!
//! Replace `SSTORE` opcode gas cost calculation (including refunds) with
//! the following logic:
//!
//! * If *gasleft* is less than or equal to gas stipend, fail the current
//!   call frame with 'out of gas' exception.
//! * If *current value* equals *new value* (this is a no-op), `SLOAD_GAS`
//!   is deducted.
//! * If *current value* does not equal *new value*
//!   * If *original value* equals *current value* (this storage slot has
//!     not been changed by the current execution context)
//!     * If *original value* is 0, `SSTORE_SET_GAS` is deducted.
//!     * Otherwise, `SSTORE_RESET_GAS` gas is deducted. If *new value* is
//!       0, add `SSTORE_CLEARS_SCHEDULE` gas to refund counter.
//!   * If *original value* does not equal *current value* (this storage
//!     slot is dirty), `SLOAD_GAS` gas is deducted. Apply both of the
//!     following clauses.
//!     * If *original value* is not 0
//!       * If *current value* is 0 (also means that *new value* is not
//!         0), remove `SSTORE_CLEARS_SCHEDULE` gas from refund
//!         counter.
//!       * If *new value* is 0 (also means that *current value* is not
//!         0), add `SSTORE_CLEARS_SCHEDULE` gas to refund counter.
//!     * If *original value* equals *new value* (this storage slot is
//!       reset)
//!       * If *original value* is 0, add `SSTORE_SET_GAS - SLOAD_GAS` to
//!         refund counter.
//!       * Otherwise, add `SSTORE_RESET_GAS - SLOAD_GAS` gas to refund
//!         counter.
//!
//! An implementation should also note that with the above definition, if
//! the implementation uses call-frame refund counter, the counter can go
//! negative. If the implementation uses transaction-wise refund counter,
//! the counter always stays positive.
//!
//! ## Rationale
//!
//! This EIP mostly achieves what a transient storage tries to do
//! ([EIP-1087] and [EIP-1153]), but without the complexity of introducing the
//! concept of "dirty maps", or an extra storage struct.
//!
//! * We don't suffer from the optimization limitation of
//!   EIP-1087. EIP-1087 requires keeping a dirty map for storage changes,
//!   and implicitly makes the assumption that a transaction's storage
//!   changes are committed to the storage trie at the end of a
//!   transaction. This works well for some implementations, but not for
//!   others. After [EIP-658], an efficient storage cache implementation
//!   would probably use an in-memory trie (without RLP encoding/decoding)
//!   or other immutable data structures to keep track of storage changes,
//!   and only commit changes at the end of a block. For them, it is
//!   possible to know a storage's original value and current value, but
//!   it is not possible to iterate over all storage changes without
//!   incurring additional memory or processing costs.
//! * It never costs more gas compared with the current scheme.
//! * It covers all usages for a transient storage. Clients that are easy
//!   to implement EIP-1087 will also be easy to implement this
//!   specification. Some other clients might require a little bit extra
//!   refactoring on this. Nonetheless, no extra memory or processing cost
//!   is needed on runtime.
//!
//! Regarding `SSTORE` gas cost and refunds, see Appendix for proofs of
//! properties that this EIP satisfies.
//!
//! * For *absolute gas used* (that is, actual *gas used* minus *refund*),
//!   this EIP is equivalent to EIP-1087 for all cases.
//! * For one particular case, where a storage slot is changed, reset to
//!   its original value, and then changed again, EIP-1283 would move more
//!   gases to refund counter compared with EIP-1087.
//!
//! Examine examples provided in EIP-1087's Motivation (with `SLOAD_GAS` being
//! `200`):
//!
//! * If a contract with empty storage sets slot 0 to 1, then back to 0,
//!   it will be charged `20000 + 200 - 19800 = 400` gas.
//! * A contract with empty storage that increments slot 0 5 times will be
//!   charged `20000 + 5 * 200 = 21000` gas.
//! * A balance transfer from account A to account B followed by a
//!   transfer from B to C, with all accounts having nonzero starting and
//!   ending balances, it will cost `5000 * 3 + 200 - 4800 = 10400` gas.
//!
//! In order to keep in place the implicit reentrancy protection of
//! existing contracts, transactions should not be allowed to modify state
//! if the remaining gas is lower then the gas stipend given to
//! "transfer"/"send" in Solidity. These are other proposed remediations
//! and objections to implementing them:
//!
//! * Drop EIP-1283 and abstain from modifying `SSTORE` cost
//!   * EIP-1283 is an important update
//!   * It was accepted and implemented on test networks and in clients.
//! * Add a new call context that permits LOG opcodes but not changes to state.
//!   * Adds another call type beyond existing regular/staticcall
//! * Raise the cost of `SSTORE` to dirty slots to >=2300 gas
//!   * Makes net gas metering much less useful.
//! * Reduce the gas stipend
//!   * Makes the stipend almost useless.
//! * Increase the cost of writes to dirty slots back to 5000 gas, but add
//!   4800 gas to the refund counter
//!   * Still doesn’t make the invariant explicit.
//!   * Requires callers to supply more gas, just to have it refunded
//! * Add contract metadata specifying per-contract EVM version, and only
//!   apply `SSTORE` changes to contracts deployed with the new version.
//!
//! ## Backwards Compatibility
//!
//! This EIP requires a hard fork to implement. No gas cost increase is
//! anticipated, and many contracts will see gas reduction.
//!
//! Performing `SSTORE` has never been possible with less than 5000 gas, so
//! it does not introduce incompatibility to the Ethereum Mainnet. Gas
//! estimation should account for this requirement.
//!
//! ## Test Cases
//!
//! | Code                               | Used Gas | Refund | Original | 1st | 2nd | 3rd |
//! |------------------------------------|----------|--------|----------|-----|-----|-----|
//! | `0x60006000556000600055`           | 1612     | 0      | 0        | 0   | 0   |     |
//! | `0x60006000556001600055`           | 20812    | 0      | 0        | 0   | 1   |     |
//! | `0x60016000556000600055`           | 20812    | 19200  | 0        | 1   | 0   |     |
//! | `0x60016000556002600055`           | 20812    | 0      | 0        | 1   | 2   |     |
//! | `0x60016000556001600055`           | 20812    | 0      | 0        | 1   | 1   |     |
//! | `0x60006000556000600055`           | 5812     | 15000  | 1        | 0   | 0   |     |
//! | `0x60006000556001600055`           | 5812     | 4200   | 1        | 0   | 1   |     |
//! | `0x60006000556002600055`           | 5812     | 0      | 1        | 0   | 2   |     |
//! | `0x60026000556000600055`           | 5812     | 15000  | 1        | 2   | 0   |     |
//! | `0x60026000556003600055`           | 5812     | 0      | 1        | 2   | 3   |     |
//! | `0x60026000556001600055`           | 5812     | 4200   | 1        | 2   | 1   |     |
//! | `0x60026000556002600055`           | 5812     | 0      | 1        | 2   | 2   |     |
//! | `0x60016000556000600055`           | 5812     | 15000  | 1        | 1   | 0   |     |
//! | `0x60016000556002600055`           | 5812     | 0      | 1        | 1   | 2   |     |
//! | `0x60016000556001600055`           | 1612     | 0      | 1        | 1   | 1   |     |
//! | `0x600160005560006000556001600055` | 40818    | 19200  | 0        | 1   | 0   | 1   |
//! | `0x600060005560016000556000600055` | 10818    | 19200  | 1        | 0   | 1   | 0   |
//!
//! ## Implementation
//!
//! To be added.
//!
//! ## Appendix: Proof
//!
//! Because the *storage slot's original value* is defined as the value
//! when a reversion happens on the *current transaction*, it's easy to
//! see that call frames won't interfere `SSTORE` gas calculation. So
//! although the below proof is discussed without call frames, it applies
//! to all situations with call frames. We will discuss the case
//! separately for *original value* being zero and not zero, and use
//! *induction* to prove some properties of `SSTORE` gas cost.
//!
//! *Final value* is the value of a particular storage slot at the end of
//! a transaction. *Absolute gas used* is the absolute value of *gas used*
//! minus *refund*. We use `N` to represent the total number of `SSTORE`
//! operations on a storage slot. For states discussed below, refer to
//! *State Transition* in *Explanation* section.
//!
//! Below we do the proof under the assumption that all parameters are
//! unchanged, meaning `SLOAD_GAS` is `200`. However, note that the proof
//! still applies no matter how `SLOAD_GAS` is changed.
//!
//! ### Original Value Being Zero
//!
//! When *original value* is 0, we want to prove that:
//!
//! * **Case I**: If the *final value* ends up still being 0, we want to charge `200 *
//!   N` gases, because no disk write is needed.
//! * **Case II**: If the *final value* ends up being a non-zero value, we want to
//!   charge `20000 + 200 * (N-1)` gas, because it requires writing this
//!   slot to disk.
//!   
//! #### Base Case
//!
//! We always start at state A. The first `SSTORE` can:
//!
//! * Go to state A: 200 gas is deducted. We satisfy *Case I* because
//!   `200 * N == 200 * 1`.
//! * Go to state B: 20000 gas is deducted. We satisfy *Case II* because
//!   `20000 + 200 * (N-1) == 20000 + 200 * 0`.
//!   
//! #### Inductive Step
//!
//! * From A to A. The previous gas cost is `200 * (N-1)`. The current
//!   gas cost is `200 + 200 * (N-1)`. It satisfy *Case I*.
//! * From A to B. The previous gas cost is `200 * (N-1)`. The current
//!   gas cost is `20000 + 200 * (N-1)`. It satisfy *Case II*.
//! * From B to B. The previous gas cost is `20000 + 200 * (N-2)`. The
//!   current gas cost is `200 + 20000 + 200 * (N-2)`. It satisfy
//!   *Case II*.
//! * From B to A. The previous gas cost is `20000 + 200 * (N-2)`. The
//!   current gas cost is `200 - 19800 + 20000 + 200 * (N-2)`. It satisfy
//!   *Case I*.
//!   
//! ### Original Value Not Being Zero
//!
//! When *original value* is not 0, we want to prove that:
//!
//! * **Case I**: If the *final value* ends up unchanged, we want to
//!   charge `200 * N` gases, because no disk write is needed.
//! * **Case II**: If the *final value* ends up being zero, we want to
//!   charge `5000 - 15000 + 200 * (N-1)` gas. Note that `15000` is the
//!   refund in actual definition.
//! * **Case III**: If the *final value* ends up being a changed non-zero
//!   value, we want to charge `5000 + 200 * (N-1)` gas.
//!   
//! #### Base Case
//!
//! We always start at state X. The first `SSTORE` can:
//!
//! * Go to state X: 200 gas is deducted. We satisfy *Case I* because
//!   `200 * N == 200 * 1`.
//! * Go to state Y: 5000 gas is deducted. We satisfy *Case III* because
//!   `5000 + 200 * (N-1) == 5000 + 200 * 0`.
//! * Go to state Z: The absolute gas used is `5000 - 15000` where 15000
//!   is the refund. We satisfy *Case II* because `5000 - 15000 + 200 *
//!   (N-1) == 5000 - 15000 + 200 * 0`.
//!   
//! #### Inductive Step
//!
//! * From X to X. The previous gas cost is `200 * (N-1)`. The current gas
//!   cost is `200 + 200 * (N-1)`. It satisfy *Case I*.
//! * From X to Y. The previous gas cost is `200 * (N-1)`. The current gas
//!   cost is `5000 + 200 * (N-1)`. It satisfy *Case III*.
//! * From X to Z. The previous gas cost is `200 * (N-1)`. The current
//!   absolute gas cost is `5000 - 15000 + 200 * (N-1)`. It satisfy *Case
//!   II*.
//! * From Y to X. The previous gas cost is `5000 + 200 * (N-2)`. The
//!   absolute current gas cost is `200 - 4800 + 5000 + 200 * (N-2)`. It
//!   satisfy *Case I*.
//! * From Y to Y. The previous gas cost is `5000 + 200 * (N-2)`. The
//!   current gas cost is `200 + 5000 + 200 * (N-2)`. It satisfy *Case
//!   III*.
//! * From Y to Z. The previous gas cost is `5000 + 200 * (N-2)`. The
//!   current absolute gas cost is `200 - 15000 + 5000 + 200 * (N-2)`. It
//!   satisfy *Case II*.
//! * From Z to X. The previous gas cost is `5000 - 15000 + 200 *
//!   (N-2)`. The current absolute gas cost is `200 + 10200 + 5000 -
//!   15000 + 200 * (N-2)`. It satisfy *Case I*.
//! * From Z to Y. The previous gas cost is `5000 - 15000 + 200 *
//!   (N-2)`. The current absolute gas cost is `200 + 15000 + 5000 -
//!   15000 + 200 * (N-2)`. It satisfy *Case III*.
//! * From Z to Z. The previous gas cost is `5000 - 15000 + 200 *
//!   (N-2)`. The current absolute gas cost is `200 + 5000 - 15000 + 200 *
//!   (N-2)`. It satisfy *Case II*.
//!
//! Wei Tang (@sorpaas), "EIP-2200: Structured Definitions for Net Gas Metering," Ethereum Improvement Proposals, no. 2200, July 2019. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-2200>.

use crate::eip::Eip;

/// EIP-220: Structured Definitions for Net Gas Metering.
pub struct Eip2200;

impl Eip for Eip2200 {
    const NUMBER: u32 = 2200;
}
