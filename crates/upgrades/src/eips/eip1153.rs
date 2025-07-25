//! Transient storage opcodes.
//!
//! ## Abstract
//!
//! This proposal introduces transient storage opcodes, which manipulate state that behaves identically to storage, except that transient storage is discarded after every transaction, and `TSTORE` is not subject to the gas stipend check as defined in [EIP-2200](./eip-2200.md). In other words, the values of transient storage are never deserialized from storage or serialized to storage. Thus transient storage is cheaper since it never requires disk access. Transient storage is accessible to smart contracts via 2 new opcodes, `TLOAD` and `TSTORE`, where “T” stands for "transient:"
//!
//! ```python
//! TLOAD  (0x5c)
//! TSTORE (0x5d)
//! ```
//!
//! ## Motivation
//!
//! Running a transaction in Ethereum can generate multiple nested frames of execution, each created by `CALL` (or similar) instructions. Contracts can be re-entered during the same transaction, in which case there are more than one frame belonging to one contract. Currently, these frames can communicate in two ways: via inputs/outputs passed via `CALL` instructions, and via storage updates. If there is an intermediate frame belonging to another untrusted contract, communication via inputs/outputs is not secure. Notable example is a reentrancy lock which cannot rely on the intermediate frame to pass through the state of the lock. Communication via storage (`SSTORE`/`SLOAD`) is costly. Transient storage is a dedicated and gas efficient solution to the problem of inter frame communication.
//!
//! Storage refunds accumulated due to inter frame communication are also limited to 20% of gas spent by a transaction due to [EIP-3529](./eip-3529.md) (introduced in the London hard fork). This greatly reduces the refunds for transiently-set storage slots in otherwise low-cost transactions. For example, in order to receive the full refund of one re-entrancy lock, the transaction must spend ~80k gas on other operations.
//!
//! Language support could be added in relatively easy way. For example, in Solidity, a qualifier `transient` can be introduced (similar to the existing qualifiers `memory` and `storage`, and Java's own `transient` keyword with a similar meaning). Since the addressing scheme of `TSTORE` and `TLOAD` is the same as for `SSTORE` and `SLOAD`, code generation routines that exist for storage variables, can be easily generalised to also support transient storage.
//!
//! Potential use cases enabled or improved by this EIP include:
//!
//! 1. Reentrancy locks
//! 2. On-chain computable CREATE2 addresses: constructor arguments are read from the factory contract instead of passed as part of init code hash
//! 3. Single transaction [ERC-20](./eip-20.md) approvals, e.g. `#temporaryApprove(address spender, uint256 amount)`
//! 4. Fee-on-transfer contracts: pay a fee to a token contract to unlock transfers for the duration of a transaction
//! 5. "Till" pattern: allowing users to perform all actions as part of a callback, and checking the "till" is balanced at the end
//! 6. Proxy call metadata: pass additional metadata to an implementation contract without using calldata, e.g. values of immutable proxy constructor arguments  
//!
//! These opcodes are more efficient to execute than the `SSTORE` and `SLOAD` opcodes because the original value never needs to be loaded from storage (i.e. is always 0). The gas accounting rules are also simpler, since no refunds are required.
//!
//! ## Specification
//!
//! Two new opcodes are added to EVM, `TLOAD` (`0x5c`) and `TSTORE` (`0x5d`). (Note that previous drafts of this EIP specified the values `0xb3` and `0xb4` for `TLOAD` and `TSTORE` respectively to avoid conflict with other EIPs. The conflict has since been removed.)
//!
//! They use the same arguments on stack as `SLOAD` (`0x54`) and `SSTORE` (`0x55`).
//!
//! `TLOAD` pops one 32-byte word from the top of the stack, treats this value as the address, fetches 32-byte word from the transient storage at that address, and pushes the value on top of the stack.
//!
//! `TSTORE` pops two 32-byte words from the top of the stack. The word on the top is the address, and the next is the value. `TSTORE` saves the value at the given address in the transient storage.
//!
//! Addressing is the same as `SLOAD` and `SSTORE`. i.e. each 32-byte address points to a unique 32-byte word.
//!
//! Gas cost for `TSTORE` is the same as a warm `SSTORE` of a dirty slot (i.e. original value is not new value and is not current value, currently 100 gas), and gas cost of `TLOAD` is the same as a hot `SLOAD` (value has been read before, currently 100 gas). Gas cost cannot be on par with memory access due to transient storage's interactions with reverts.
//!
//! All values in transient storage are discarded at the end of the transaction.
//!
//! Transient storage is private to the contract that owns it, in the same way as persistent storage. Only owning contract frames may access their transient storage. And when they do, all the frames access the same transient store, in the same way as persistent storage, but unlike memory.
//!
//! When transient storage is used in the context of `DELEGATECALL` or `CALLCODE`, then the owning contract of the transient storage is the contract that issued `DELEGATECALL` or `CALLCODE` instruction (the caller) as with persistent storage. When transient storage is used in the context of `CALL` or `STATICCALL`, then the owning contract of the transient storage is the contract that is the target of the `CALL` or `STATICCALL` instruction (the callee).
//!
//! If a frame reverts, all writes to transient storage that took place between entry to the frame and the return are reverted, including those that took place in inner calls.  This mimics the behavior of persistent storage.
//!
//! If the `TSTORE` opcode is called within the context of a `STATICCALL`, it will result in an exception instead of performing the modification. `TLOAD` is allowed within the context of a `STATICCALL`.
//!
//! The behavior of the opcodes for transient storage differs from the opcodes for storage in that `TSTORE` does not require _gasleft_, as defined in [EIP-2200](./eip-2200.md), to be less than or equal to the gas stipend (currently 2,300).
//!
//! ## Rationale
//!
//! Another option to solve the problem of inter-frame communication is repricing the `SSTORE` and `SLOAD` opcodes to be cheaper for the transient storage use case. This has already been done as of [EIP-2200](./eip-2200.md). However, [EIP-3529](./eip-3529.md) reduced the maximum refund to only 20% of the transaction gas cost, which means the use of transient storage is severely limited.
//!
//! Another approach is to keep the refund counter for transient storage separate from the refund counter for other storage uses, and remove the refund cap for transient storage. However, that approach is more complex to implement and understand. For example, the 20% refund cap must be applied to the gas used _after_ subtracting the uncapped gas refund. Otherwise, the refund amount available subject to the 20% refund cap could be increased by executing transient storage writes. Thus it is preferable to have a separate mechanism that does not interact with the refund counter. Future hard forks can remove the complex refund behavior meant to support the transient storage use case, encouraging migration to contracts that are more efficient for the Ethereum clients to execute.
//!
//! There is a known objection to the word-addressed storage-like interface of the `TSTORE` and `TLOAD` opcodes since transient storage is more akin to memory than storage in lifecycle. A byte-addressed memory-like interface is another option. The storage-like word-addressed interface is preferred due to the usefulness of mappings in combination with the transaction-scoped memory region. Often times, you will need to keep transient state with arbitrary keys, such as in the [ERC-20](./eip-20.md) temporary approval use case which uses a mapping of `(owner, spender)` to `allowance`. Mappings are difficult to implement using linear memory, and linear memory must also have dynamic gas costs. It is also more complicated to handle reverts with a linear memory. It is possible to have a memory-like interface while the underlying implementation uses a map to allow for storage in arbitrary offsets, but this would result in a third memory-storage hybrid interface that would require new code paths in compilers.
//!
//! Some think that a unique transaction identifier may obviate the need for transient storage as described in this EIP. This is a misconception: a transaction identifier used in combination with regular storage has all the same issues that motivate this EIP. The two features are orthogonal.  
//!
//! Relative cons of this transient storage EIP:
//!
//! - Does not address transient usages of storage in existing contracts
//! - New code in the clients
//! - New concept for the yellow paper (more to update)
//!
//! Relative pros of this transient storage EIP:
//!
//! - Transient storage opcodes are considered separately in protocol upgrades and not inadvertently broken (e.g. [EIP-3529](./eip-3529.md))
//! - Clients do not need to load the original value
//! - No upfront gas cost to account for non-transient writes
//! - Does not change the semantics of the existing operations
//! - No need to clear storage slots after usage
//! - Simpler gas accounting rules
//! - Future storage designs (e.g. Verkle tree) do not need to account for transient storage refunds
//!
//! ## Backwards Compatibility
//!
//! This EIP requires a hard fork to implement.
//!
//! Since this EIP does not change behavior of any existing opcodes, it is backwards compatible with all existing smart contracts.
//!
//! ## Test Cases
//!
//! A test suite for this EIP can be found [here](https://github.com/ethereum/execution-spec-tests/tree/1983444bbe1a471886ef7c0e82253ffe2a4053e1/tests/cancun/eip1153_tstore).
//!
//! ## Reference Implementation
//!
//! Because the transient storage must behave almost identically to storage within the context of a single transaction with regards to revert behavior, it is necessary to be able to revert to a previous state of transient storage within a transaction. At the same time reverts are exceptional cases and loads, stores and returns should be cheap.
//!
//! A map of current state plus a journal of all changes and a list of checkpoints is recommended. This has the following time complexities:
//!
//! - On entry to a call frame, a call marker is added to the list - `O(1)`
//! - New values are written to the current state, and the previous value is written to the journal - `O(1)`
//! - When a call exits successfully, the marker to the journal index of when that call was entered is discarded - `O(1)`
//! - On revert all entries are reverted up to the last checkpoint, in reverse - `O(N)` where `N` = number of journal entries since last checkpoint
//!
//! ```typescript
//! interface JournalEntry {
//!     addr: string
//!     key: string
//!     prevValue: string
//! }
//!
//! type Journal = JournalEntry[]
//!
//! type Checkpoints = Journal['length'][]
//!
//! interface Current {
//!     [addr: string]: {
//!         [key: string]: string
//!     }
//! }
//!
//! const EMPTY_VALUE = '0x0000000000000000000000000000000000000000000000000000000000000000'
//!
//! class TransientStorage {
//!     /**
//!      * The current state of transient storage.
//!      */
//!     private current: Current = {}
//!     /**
//!      * All changes are written to the journal. On revert, we apply the changes in reverse to the last checkpoint.
//!      */
//!     private journal: Journal = []
//!     /**
//!      * The length of the journal at the time of each checkpoint
//!      */
//!     private checkpoints: Checkpoints = [0]
//!
//!     /**
//!      * Returns the current value of the given contract address and key
//!      * @param addr The address of the contract
//!      * @param key The key of transient storage for the address
//!      */
//!     public get(addr: string, key: string): string {
//!         return this.current[addr]?.[key] ?? EMPTY_VALUE
//!     }
//!
//!     /**
//!      * Set the current value in the map
//!      * @param addr the address of the contract for which the key is being set
//!      * @param key the slot to set for the address
//!      * @param value the new value of the slot to set
//!      */
//!     public put(addr: string, key: string, value: string) {
//!         this.journal.push({
//!             addr,
//!             key,
//!             prevValue: this.get(addr, key),
//!         })
//!
//!         this.current[addr] = this.current[addr] ?? {}
//!         this.current[addr][key] = value;
//!     }
//!
//!     /**
//!      * Commit all the changes since the last checkpoint
//!      */
//!     public commit(): void {
//!         if (this.checkpoints.length === 0) throw new Error('Nothing to commit')
//!         this.checkpoints.pop() // The last checkpoint is discarded.
//!     }
//!
//!     /**
//!      * To be called whenever entering a new context. If revert is called after checkpoint, all changes made after the latest checkpoint are reverted.
//!      */
//!     public checkpoint(): void {
//!         this.checkpoints.push(this.journal.length)
//!     }
//!
//!     /**
//!      * Revert transient storage to the state from the last call to checkpoint
//!      */
//!     public revert() {
//!         const lastCheckpoint = this.checkpoints.pop()
//!         if (typeof lastCheckpoint === 'undefined') throw new Error('Nothing to revert')
//!
//!         for (let i = this.journal.length - 1; i >= lastCheckpoint; i--) {
//!             const {addr, key, prevValue} = this.journal[i]
//!             // we can assume it exists, since it was written in the journal
//!             this.current[addr][key] = prevValue
//!         }
//!         this.journal.splice(lastCheckpoint, this.journal.length - lastCheckpoint)
//!     }
//! }
//! ```
//!
//! The worst case time complexity can be produced by writing the maximum number of keys that can fit in one block, and then reverting. In this case, the client is required to do twice as many writes to apply all the entries in the journal. However, the same case applies to the state journaling implementation of existing clients, and cannot be DOS'd with the following code.
//!
//! ```solidity
//! pragma solidity =0.8.13;
//!
//! contract TryDOS {
//!     uint256 slot;
//!
//!     constructor() {
//!         slot = 1;
//!     }
//!
//!     function tryDOS() external {
//!         uint256 i = 1;
//!         while (gasleft() > 5000) {
//!             unchecked {
//!                 slot = i++;
//!             }
//!         }
//!         revert();
//!     }
//! }
//! ```
//!
//! ## Security Considerations
//!
//! `TSTORE` presents a new way to allocate memory on a node with linear cost. In other words, each TSTORE allows the developer to store 32 bytes for 100 gas, excluding any other required operations to prepare the stack. Given 30 million gas, the maximum amount of memory that can be allocated using TSTORE is:
//!
//! ```python
//! 30M gas * 1 TSTORE / 100 gas * 32 bytes / 1 TSTORE * 1MB / 2^20 bytes ~= 9.15MB
//! ```
//!
//! Given the same amount of gas, the maximum amount of memory that can be allocated in a single context by `MSTORE` is ~3.75MB:
//!
//! ```python
//! 30M gas = 3x + x^2 / 512 => x = ~123,169 32-byte words
//! ~123,169 words * 32 bytes/word * 1MB / 2^20 bytes = 3.75MB
//! ```
//!
//! However, if you only spend 1M gas allocating memory in each context, and make calls to reset the memory expansion cost, you can allocate ~700KB per million gas, for a total of ~20MB of memory allocated:
//!
//! ```python
//! 1M gas = 3x + x^2 / 512 => x = ~21,872 32-byte words
//! 30M gas * ~21,872 words / 1M gas * 32 bytes/word * 1MB / 2^20 bytes = ~20MB
//! ```
//!
//! Smart contract developers should understand the lifetime of transient storage variables before use. Because transient storage is automatically cleared at the end of the transaction, smart contract developers may be tempted to avoid clearing slots as part of a call in order to save gas. However, this could prevent further interactions with the contract in the same transaction (e.g. in the case of re-entrancy locks) or cause other bugs, so smart contract developers should be careful to _only_ leave transient storage slots with nonzero values when those slots are intended to be used by future calls within the same transaction. Otherwise, these opcodes behave exactly the same as `SSTORE` and `SLOAD`, so all the usual security considerations apply especially in regard to reentrancy risk.
//!
//! Smart contract developers may also be tempted to use transient storage as an alternative to in-memory mappings. They should be aware that transient storage is not discarded when a call returns or reverts, as is memory, and should prefer memory for these use cases so as not to create unexpected behavior on reentrancy in the same transaction. The necessarily high cost of transient storage over memory should already discourage this usage pattern. Most usages of in-memory mappings can be better implemented with key-sorted lists of entries, and in-memory mappings are rarely required in smart contracts (i.e. the author knows of no known use cases in production).
//!
//! Alexey Akhunov (@`AlexeyAkhunov`), Moody Salem (@moodysalem), "EIP-1153: Transient storage opcodes," Ethereum Improvement Proposals, no. 1153, June 2018. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-1153>.

use asm::instruction::{TLoad, TStore};

use crate::eip::{Eip, macros::introduces_instructions};

/// Transient storage opcodes.
pub struct Eip1153;

impl Eip for Eip1153 {
    const NUMBER: u32 = 1153;
}

introduces_instructions!(Eip1153, TLoad, TStore);
