//! EIP-4788: Beacon block root in the EVM.
//!
//! ## Abstract
//!
//! Commit to the hash tree root of each beacon chain block in the corresponding execution payload header.
//!
//! Store each of these roots in a smart contract.
//!
//! ## Motivation
//!
//! Roots of the beacon chain blocks are cryptographic accumulators that allow proofs of arbitrary consensus state.
//! Exposing these roots inside the EVM allows for trust-minimized access to the consensus layer.
//! This functionality supports a wide variety of use cases that improve trust assumptions of staking pools,
//! restaking constructions, smart contract bridges, MEV mitigations and more.
//!
//! ## Specification
//!
//! | constants                    | value                                        |
//! |---                           |---                                           |
//! | `FORK_TIMESTAMP`             | `1710338135`                                          |
//! | `HISTORY_BUFFER_LENGTH`      | `8191`                                       |
//! | `SYSTEM_ADDRESS`             | `0xfffffffffffffffffffffffffffffffffffffffe` |
//! | `BEACON_ROOTS_ADDRESS`       | `0x000F3df6D732807Ef1319fB7B8bB8522d0Beac02` |
//!
//! ### Background
//!
//! The high-level idea is that each execution block contains the parent beacon block's root. Even in the event of missed slots since the previous block root does not change,
//! we only need a constant amount of space to represent this "oracle" in each execution block. To improve the usability of this oracle, a small history of block roots
//! are stored in the contract.
//!
//! To bound the amount of storage this construction consumes, a ring buffer is used that mirrors a block root accumulator on the consensus layer.
//!
//! ### Block structure and validity
//!
//! Beginning at the execution timestamp `FORK_TIMESTAMP`, execution clients **MUST** extend the header schema with an additional field: the `parent_beacon_block_root`.
//! This root consumes 32 bytes and is exactly the [hash tree root](https://github.com/ethereum/consensus-specs/blob/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz/simple-serialize.md#merkleization) of the parent beacon block for the given execution block.
//!
//! The resulting RLP encoding of the header is therefore:
//!
//! ```python
//! rlp([
//!     parent_hash,
//!     0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347, # ommers hash
//!     coinbase,
//!     state_root,
//!     txs_root,
//!     receipts_root,
//!     logs_bloom,
//!     0, # difficulty
//!     number,
//!     gas_limit,
//!     gas_used,
//!     timestamp,
//!     extradata,
//!     prev_randao,
//!     0x0000000000000000, # nonce
//!     base_fee_per_gas,
//!     withdrawals_root,
//!     blob_gas_used,
//!     excess_blob_gas,
//!     parent_beacon_block_root,
//! ])
//! ```
//!
//! Validity of the parent beacon block root is guaranteed from the consensus layer, much like how withdrawals are handled.
//!
//! When verifying a block, execution clients **MUST** ensure the root value in the block header matches the one provided by the consensus client.
//!
//! For a genesis block with no existing parent beacon block root the 32 zero bytes are used as a root placeholder.
//!
//! #### Beacon roots contract
//!
//! The beacon roots contract has two operations: `get` and `set`. The input itself is not used to determine which function to execute, for that the result of `caller` is used. If `caller` is equal to `SYSTEM_ADDRESS` then the operation to perform is `set`. Otherwise, `get`.
//!
//! ##### `get`
//!
//! * Callers provide the `timestamp` they are querying encoded as 32 bytes in big-endian format.
//! * If the input is not exactly 32 bytes, the contract must revert.
//! * If the input is equal to 0, the contract must revert.
//! * Given `timestamp`, the contract computes the storage index in which the timestamp is stored by computing the modulo `timestamp % HISTORY_BUFFER_LENGTH` and reads the value.
//! * If the `timestamp` does not match, the contract must revert.
//! * Finally, the beacon root associated with the timestamp is returned to the user. It is stored at `timestamp % HISTORY_BUFFER_LENGTH + HISTORY_BUFFER_LENGTH`.
//!
//! ##### `set`
//!
//! * Caller provides the parent beacon block root as calldata to the contract.
//! * Set the storage value at `header.timestamp % HISTORY_BUFFER_LENGTH` to be `header.timestamp`
//! * Set the storage value at `header.timestamp % HISTORY_BUFFER_LENGTH + HISTORY_BUFFER_LENGTH` to be `calldata[0:32]`
//!
//! ##### Pseudocode
//!
//! ```python
//! if evm.caller == SYSTEM_ADDRESS:
//!     set()
//! else:
//!     get()
//!
//! def get():
//!     if len(evm.calldata) != 32:
//!         evm.revert()
//!
//!     if to_uint256_be(evm.calldata) == 0:
//!         evm.revert()
//!
//!     timestamp_idx = to_uint256_be(evm.calldata) % HISTORY_BUFFER_LENGTH
//!     timestamp = storage.get(timestamp_idx)
//!
//!     if timestamp != evm.calldata:
//!         evm.revert()
//!
//!     root_idx = timestamp_idx + HISTORY_BUFFER_LENGTH
//!     root = storage.get(root_idx)
//!
//!     evm.return(root)
//!
//! def set():
//!     timestamp_idx = to_uint256_be(evm.timestamp) % HISTORY_BUFFER_LENGTH
//!     root_idx = timestamp_idx + HISTORY_BUFFER_LENGTH
//!
//!     storage.set(timestamp_idx, evm.timestamp)
//!     storage.set(root_idx, evm.calldata)
//! ```
//!
//! ##### Bytecode
//!
//! The exact contract bytecode is shared below.
//!
//! ```asm
//! caller
//! push20 0xfffffffffffffffffffffffffffffffffffffffe
//! eq
//! push1 0x4d
//! jumpi
//!
//! push1 0x20
//! calldatasize
//! eq
//! push1 0x24
//! jumpi
//!
//! push0
//! push0
//! revert
//!
//! jumpdest
//! push0
//! calldataload
//! dup1
//! iszero
//! push1 0x49
//! jumpi
//!
//! push3 0x001fff
//! dup2
//! mod
//! swap1
//! dup2
//! sload
//! eq
//! push1 0x3c
//! jumpi
//!
//! push0
//! push0
//! revert
//!
//! jumpdest
//! push3 0x001fff
//! add
//! sload
//! push0
//! mstore
//! push1 0x20
//! push0
//! return
//!
//! jumpdest
//! push0
//! push0
//! revert
//!
//! jumpdest
//! push3 0x001fff
//! timestamp
//! mod
//! timestamp
//! dup2
//! sstore
//! push0
//! calldataload
//! swap1
//! push3 0x001fff
//! add
//! sstore
//! stop
//! ```
//!
//! #### Deployment
//!
//! The beacon roots contract is deployed like any other smart contract. A special synthetic address is generated
//! by working backwards from the desired deployment transaction:
//!
//! ```json
//! {
//!   "type": "0x0",
//!   "nonce": "0x0",
//!   "to": null,
//!   "gas": "0x3d090",
//!   "gasPrice": "0xe8d4a51000",
//!   "maxPriorityFeePerGas": null,
//!   "maxFeePerGas": null,
//!   "value": "0x0",
//!   "input": "0x60618060095f395ff33373fffffffffffffffffffffffffffffffffffffffe14604d57602036146024575f5ffd5b5f35801560495762001fff810690815414603c575f5ffd5b62001fff01545f5260205ff35b5f5ffd5b62001fff42064281555f359062001fff015500",
//!   "v": "0x1b",
//!   "r": "0x539",
//!   "s": "0x1b9b6eb1f0",
//!   "hash": "0xdf52c2d3bbe38820fff7b5eaab3db1b91f8e1412b56497d88388fb5d4ea1fde0"
//! }
//! ```
//!
//! Note, the input in the transaction has a simple constructor prefixing the desired runtime code.
//!
//! The sender of the transaction can be calculated as `0x0B799C86a49DEeb90402691F1041aa3AF2d3C875`. The address of the first contract deployed from the account is `rlp([sender, 0])` which equals `0x000F3df6D732807Ef1319fB7B8bB8522d0Beac02`. This is how `BEACON_ROOTS_ADDRESS` is determined. Although this style of contract creation is not tied to any specific initcode like create2 is, the synthetic address is cryptographically bound to the input data of the transaction (e.g. the initcode).
//!
//! ### Block processing
//!
//! At the start of processing any execution block where `block.timestamp >= FORK_TIMESTAMP` (i.e. before processing any transactions), call `BEACON_ROOTS_ADDRESS` as `SYSTEM_ADDRESS` with the 32-byte input of `header.parent_beacon_block_root`, a gas limit of `30_000_000`, and `0` value. This will trigger the `set()` routine of the beacon roots contract. This is a system operation and therefore:
//!
//! * the call must execute to completion
//! * the call does not count against the block's gas limit
//! * the call does not follow the [EIP-1559](./eip-1559.md) burn semantics - no value should be transferred as part of the call
//! * if no code exists at `BEACON_ROOTS_ADDRESS`, the call must fail silently
//!
//! Clients may decide to omit an explicit EVM call and directly set the storage values. Note: While this is a valid optimization for Ethereum mainnet, it could be problematic on non-mainnet situations in case a different contract is used.
//!
//! If this EIP is active in a genesis block, the genesis header's `parent_beacon_block_root` must be `0x0` and no system transaction may occur.
//!
//! ## Rationale
//!
//! ### Why not repurpose `BLOCKHASH`?
//!
//! The `BLOCKHASH` opcode could be repurposed to provide the beacon root instead of some execution block hash.
//! To minimize code change, avoid breaking changes to smart contracts, and simplify deployment to mainnet, this EIP suggests leaving `BLOCKHASH` alone and adding new
//! functionality with the desired semantics.
//!
//! ### Beacon block root instead of state root
//!
//! Block roots are preferred over state roots so there is a constant amount of work to do with each new execution block. Otherwise, skipped slots would require
//! a linear amount of work with each new payload. While skipped slots are quite rare on mainnet, it is best to not add additional load under what would already
//! be nonfavorable conditions.
//!
//! Use of block root over state root does mean proofs will require a few additional nodes but this cost is negligible (and could be amortized across all consumers,
//! e.g. with a singleton state root contract that caches the proof per slot).
//!
//! ### Why two ring buffers?
//!
//! The first ring buffer only tracks `HISTORY_BUFFER_LENGTH` worth of roots and so for all possible timestamp values would consume a constant amount of storage.
//! However, this design opens the contract to an attack where a skipped slot that has the same value modulo the ring buffer length would return an old root value,
//! rather than the most recent one.
//!
//! To nullify this attack while retaining a fixed memory footprint, this EIP keeps track of the pair of data `(parent_beacon_block_root, timestamp)` for each index into the
//! ring buffer and verifies the timestamp matches the one originally used to write the root data when being read. Given the fixed size of storage slots (only 32 bytes), the requirement
//! to store a pair of values necessitates two ring buffers, rather than just one.
//!
//! ### Size of ring buffers
//!
//! The ring buffer data structures are sized to hold 8191 roots from the consensus layer. Using a prime number as the ring buffer size ensures that no value is overwritten until the entire ring buffer has been saturated and thereafter, each value will be updated once per iteration. This also means that even if the slot times were to change, we would continue to use at most 8191 storage slots.
//!
//! Given the current mainnet values, 8191 roots provides about a day of coverage. This gives users plenty of time to make a transaction with a verification against a specific root and get the transaction included on-chain.
//!
//! ## Backwards Compatibility
//!
//! No issues.
//!
//! Alex Stokes (@ralexstokes), Ansgar Dietrichs (@adietrichs), Danny Ryan (@djrtwo), Martin Holst Swende (@holiman), lightclient (@lightclient), "EIP-4788: Beacon block root in the EVM," Ethereum Improvement Proposals, no. 4788, February 2022. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-4788>.

use crate::eip::Eip;

/// EIP-4788: Beacon block root in the EVM.
pub struct Eip4788;

impl Eip for Eip4788 {
    const NUMBER: u32 = 4788;
}
