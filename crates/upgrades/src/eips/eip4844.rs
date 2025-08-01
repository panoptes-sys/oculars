//! EIP-4844: Shard Blob Transactions.
//!
//! ## Abstract
//!
//! Introduce a new transaction format for "blob-carrying transactions" which contain a large amount of data that cannot be
//! accessed by EVM execution, but whose commitment can be accessed.
//! The format is intended to be fully compatible with the format that will be used in full sharding.
//!
//! ## Motivation
//!
//! Rollups are in the short and medium term, and possibly in the long term, the only trustless scaling solution for Ethereum.
//! Transaction fees on L1 have been very high for months and there is greater urgency in doing anything required to help facilitate an ecosystem-wide move to rollups.
//! Rollups are significantly reducing fees for many Ethereum users: Optimism and Arbitrum frequently provide fees that are ~3-8x lower than the Ethereum base layer itself,
//! and ZK rollups, which have better data compression and can avoid including signatures, have fees ~40-100x lower than the base layer.
//!
//! However, even these fees are too expensive for many users. The long-term solution to the long-term inadequacy of rollups
//! by themselves has always been data sharding, which would add ~16 MB per block of dedicated data space to the chain that rollups could use.
//! However, data sharding will still take a considerable amount of time to finish implementing and deploying.
//!
//! This EIP provides a stop-gap solution until that point by implementing the _transaction format_ that would be used in sharding,
//! but not actually sharding those transactions. Instead, the data from this transaction format is simply part of the beacon chain and is fully downloaded
//! by all consensus nodes (but can be deleted after only a relatively short delay).
//! Compared to full data sharding, this EIP has a reduced cap on the number of these transactions that can be included, corresponding to a target of ~0.375 MB per block and a limit of ~0.75 MB.
//!
//! ## Specification
//!
//! ### Parameters
//!
//! | Constant | Value |
//! | - | - |
//! | `BLOB_TX_TYPE` | `Bytes1(0x03)` |
//! | `BYTES_PER_FIELD_ELEMENT` | `32` |
//! | `FIELD_ELEMENTS_PER_BLOB` | `4096` |
//! | `BLS_MODULUS` | `52435875175126190479447740508185965837690552500527637822603658699938581184513` |
//! | `VERSIONED_HASH_VERSION_KZG` | `Bytes1(0x01)` |
//! | `POINT_EVALUATION_PRECOMPILE_ADDRESS` | `Bytes20(0x0A)` |
//! | `POINT_EVALUATION_PRECOMPILE_GAS` | `50000` |
//! | `MAX_BLOB_GAS_PER_BLOCK` | `786432` |
//! | `TARGET_BLOB_GAS_PER_BLOCK` | `393216` |
//! | `MIN_BASE_FEE_PER_BLOB_GAS` | `1` |
//! | `BLOB_BASE_FEE_UPDATE_FRACTION` | `3338477` |
//! | `GAS_PER_BLOB` | `2**17` |
//! | `HASH_OPCODE_BYTE` | `Bytes1(0x49)` |
//! | `HASH_OPCODE_GAS` | `3` |
//! | [`MIN_EPOCHS_FOR_BLOB_SIDECARS_REQUESTS`](https://github.com/ethereum/consensus-specs/blob/4de1d156c78b555421b72d6067c73b614ab55584/configs/mainnet.yaml#L148) | `4096` |
//!
//! ### Type aliases
//!
//! | Type | Base type | Additional checks |
//! | - | - | - |
//! | `Blob` | `ByteVector[BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB]` | |
//! | `VersionedHash` | `Bytes32` | |
//! | `KZGCommitment` | `Bytes48` | Perform IETF BLS signature "`KeyValidate`" check but do allow the identity point |
//! | `KZGProof` | `Bytes48` | Same as for `KZGCommitment` |
//!
//! ### Cryptographic Helpers
//!
//! Throughout this proposal we use cryptographic methods and classes defined in the corresponding [consensus 4844 specs](https://github.com/ethereum/consensus-specs/blob/86fb82b221474cc89387fa6436806507b3849d88/specs/deneb).
//!
//! Specifically, we use the following methods from [`polynomial-commitments.md`](https://github.com/ethereum/consensus-specs/blob/86fb82b221474cc89387fa6436806507b3849d88/specs/deneb/polynomial-commitments.md):
//!
//! - [`verify_kzg_proof()`](https://github.com/ethereum/consensus-specs/blob/86fb82b221474cc89387fa6436806507b3849d88/specs/deneb/polynomial-commitments.md#verify_kzg_proof)
//! - [`verify_blob_kzg_proof_batch()`](https://github.com/ethereum/consensus-specs/blob/86fb82b221474cc89387fa6436806507b3849d88/specs/deneb/polynomial-commitments.md#verify_blob_kzg_proof_batch)
//!
//! ### Helpers
//!
//! ```python
//! def kzg_to_versioned_hash(commitment: KZGCommitment) -> VersionedHash:
//!     return VERSIONED_HASH_VERSION_KZG + sha256(commitment)[1:]
//! ```
//!
//! Approximates `factor * e ** (numerator / denominator)` using Taylor expansion:
//!
//! ```python
//! def fake_exponential(factor: int, numerator: int, denominator: int) -> int:
//!     i = 1
//!     output = 0
//!     numerator_accum = factor * denominator
//!     while numerator_accum > 0:
//!         output += numerator_accum
//!         numerator_accum = (numerator_accum * numerator) // (denominator * i)
//!         i += 1
//!     return output // denominator
//! ```
//!
//! ### Blob transaction
//!
//! We introduce a new type of [EIP-2718](./eip-2718.md) transaction, "blob transaction", where the `TransactionType` is `BLOB_TX_TYPE` and the `TransactionPayload` is the RLP serialization of the following `TransactionPayloadBody`:
//!
//! ```python
//! [chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas, gas_limit, to, value, data, access_list, max_fee_per_blob_gas, blob_versioned_hashes, y_parity, r, s]
//! ```
//!
//! The fields `chain_id`, `nonce`, `max_priority_fee_per_gas`, `max_fee_per_gas`, `gas_limit`, `value`, `data`, and `access_list` follow the same semantics as [EIP-1559](./eip-1559.md).
//!
//! The field `to` deviates slightly from the semantics with the exception that it MUST NOT be `nil` and therefore must always represent a 20-byte address. This means that blob transactions cannot have the form of a create transaction.
//!
//! The field `max_fee_per_blob_gas` is a `uint256` and the field `blob_versioned_hashes` represents a list of hash outputs from `kzg_to_versioned_hash`.
//!
//! The [EIP-2718](./eip-2718.md) `ReceiptPayload` for this transaction is `rlp([status, cumulative_transaction_gas_used, logs_bloom, logs])`.
//!
//! #### Signature
//!
//! The signature values `y_parity`, `r`, and `s` are calculated by constructing a secp256k1 signature over the following digest:
//!
//! `keccak256(BLOB_TX_TYPE || rlp([chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas, gas_limit, to, value, data, access_list, max_fee_per_blob_gas, blob_versioned_hashes]))`.
//!
//! ### Header extension
//!
//! The current header encoding is extended with two new 64-bit unsigned integer fields:
//!
//! - `blob_gas_used` is the total amount of blob gas consumed by the transactions within the block.
//! - `excess_blob_gas` is a running total of blob gas consumed in excess of the target, prior to the block. Blocks with above-target blob gas consumption increase this value, blocks with below-target blob gas consumption decrease it (bounded at 0).
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
//! ])
//! ```
//!
//! The value of `excess_blob_gas` can be calculated using the parent header.
//!
//! ```python
//! def calc_excess_blob_gas(parent: Header) -> int:
//!     if parent.excess_blob_gas + parent.blob_gas_used < TARGET_BLOB_GAS_PER_BLOCK:
//!         return 0
//!     else:
//!         return parent.excess_blob_gas + parent.blob_gas_used - TARGET_BLOB_GAS_PER_BLOCK
//! ```
//!
//! For the first post-fork block, both `parent.blob_gas_used` and `parent.excess_blob_gas` are evaluated as `0`.
//!
//! ### Gas accounting
//!
//! We introduce blob gas as a new type of gas. It is independent of normal gas and follows its own targeting rule, similar to EIP-1559.
//! We use the `excess_blob_gas` header field to store persistent data needed to compute the blob gas base fee. For now, only blobs are priced in blob gas.
//!
//! ```python
//! def calc_blob_fee(header: Header, tx: Transaction) -> int:
//!     return get_total_blob_gas(tx) * get_base_fee_per_blob_gas(header)
//!
//! def get_total_blob_gas(tx: Transaction) -> int:
//!     return GAS_PER_BLOB * len(tx.blob_versioned_hashes)
//!
//! def get_base_fee_per_blob_gas(header: Header) -> int:
//!     return fake_exponential(
//!         MIN_BASE_FEE_PER_BLOB_GAS,
//!         header.excess_blob_gas,
//!         BLOB_BASE_FEE_UPDATE_FRACTION
//!     )
//! ```
//!
//! The block validity conditions are modified to include blob gas checks (see the [Execution layer validation](#execution-layer-validation) section below).
//!
//! The actual `blob_fee` as calculated via `calc_blob_fee` is deducted from the sender balance before transaction execution and burned, and is not refunded in case of transaction failure.
//!
//! ### Opcode to get versioned hashes
//!
//! We add an instruction `BLOBHASH` (with opcode `HASH_OPCODE_BYTE`) which reads `index` from the top of the stack
//! as big-endian `uint256`, and replaces it on the stack with `tx.blob_versioned_hashes[index]`
//! if `index < len(tx.blob_versioned_hashes)`, and otherwise with a zeroed `bytes32` value.
//! The opcode has a gas cost of `HASH_OPCODE_GAS`.
//!
//! ### Point evaluation precompile
//!
//! Add a precompile at `POINT_EVALUATION_PRECOMPILE_ADDRESS` that verifies a KZG proof which claims that a blob
//! (represented by a commitment) evaluates to a given value at a given point.
//!
//! The precompile costs `POINT_EVALUATION_PRECOMPILE_GAS` and executes the following logic:
//!
//! ```python
//! def point_evaluation_precompile(input: Bytes) -> Bytes:
//!     """
//!     Verify p(z) = y given commitment that corresponds to the polynomial p(x) and a KZG proof.
//!     Also verify that the provided commitment matches the provided versioned_hash.
//!     """
//!     # The data is encoded as follows: versioned_hash | z | y | commitment | proof | with z and y being padded 32 byte big endian values
//!     assert len(input) == 192
//!     versioned_hash = input[:32]
//!     z = input[32:64]
//!     y = input[64:96]
//!     commitment = input[96:144]
//!     proof = input[144:192]
//!
//!     # Verify commitment matches versioned_hash
//!     assert kzg_to_versioned_hash(commitment) == versioned_hash
//!
//!     # Verify KZG proof with z and y in big endian format
//!     assert verify_kzg_proof(commitment, z, y, proof)
//!
//!     # Return FIELD_ELEMENTS_PER_BLOB and BLS_MODULUS as padded 32 byte big endian values
//!     return Bytes(U256(FIELD_ELEMENTS_PER_BLOB).to_be_bytes32() + U256(BLS_MODULUS).to_be_bytes32())
//! ```
//!
//! The precompile MUST reject non-canonical field elements (i.e. provided field elements MUST be strictly less than `BLS_MODULUS`).
//!
//! ### Consensus layer validation
//!
//! On the consensus layer the blobs are referenced, but not fully encoded, in the beacon block body.
//! Instead of embedding the full contents in the body, the blobs are propagated separately, as "sidecars".
//!
//! This "sidecar" design provides forward compatibility for further data increases by black-boxing `is_data_available()`:
//! with full sharding `is_data_available()` can be replaced by data-availability-sampling (DAS) thus avoiding all blobs being downloaded by all beacon nodes on the network.
//!
//! Note that the consensus layer is tasked with persisting the blobs for data availability, the execution layer is not.
//!
//! The `ethereum/consensus-specs` repository defines the following consensus layer changes involved in this EIP:
//!
//! - Beacon chain: process updated beacon blocks and ensure blobs are available.
//! - P2P network: gossip and sync updated beacon block types and new blob sidecars.
//! - Honest validator: produce beacon blocks with blobs; sign and publish the associated blob sidecars.
//!
//! ### Execution layer validation
//!
//! On the execution layer, the block validity conditions are extended as follows:
//!
//! ```python
//! def validate_block(block: Block) -> None:
//!     ...
//!
//!     # check that the excess blob gas was updated correctly
//!     assert block.header.excess_blob_gas == calc_excess_blob_gas(block.parent.header)
//!
//!     blob_gas_used = 0
//!
//!     for tx in block.transactions:
//!         ...
//!
//!         # modify the check for sufficient balance
//!         max_total_fee = tx.gas * tx.max_fee_per_gas
//!         if get_tx_type(tx) == BLOB_TX_TYPE:
//!             max_total_fee += get_total_blob_gas(tx) * tx.max_fee_per_blob_gas
//!         assert signer(tx).balance >= max_total_fee
//!
//!         ...
//!
//!         # add validity logic specific to blob txs
//!         if get_tx_type(tx) == BLOB_TX_TYPE:
//!             # there must be at least one blob
//!             assert len(tx.blob_versioned_hashes) > 0
//!
//!             # all versioned blob hashes must start with VERSIONED_HASH_VERSION_KZG
//!             for h in tx.blob_versioned_hashes:
//!                 assert h[0] == VERSIONED_HASH_VERSION_KZG
//!
//!             # ensure that the user was willing to at least pay the current blob base fee
//!             assert tx.max_fee_per_blob_gas >= get_base_fee_per_blob_gas(block.header)
//!
//!             # keep track of total blob gas spent in the block
//!             blob_gas_used += get_total_blob_gas(tx)
//!
//!     # ensure the total blob gas spent is at most equal to the limit
//!     assert blob_gas_used <= MAX_BLOB_GAS_PER_BLOCK
//!
//!     # ensure blob_gas_used matches header
//!     assert block.header.blob_gas_used == blob_gas_used
//!
//! ```
//!
//! ### Networking
//!
//! Blob transactions have two network representations. During transaction gossip responses (`PooledTransactions`), the EIP-2718 `TransactionPayload` of the blob transaction is wrapped to become:
//!
//! ```python
//! rlp([tx_payload_body, blobs, commitments, proofs])
//! ```
//!
//! Each of these elements are defined as follows:
//!
//! - `tx_payload_body` - is the `TransactionPayloadBody` of standard EIP-2718 [blob transaction](#blob-transaction)
//! - `blobs` - list of `Blob` items
//! - `commitments` - list of `KZGCommitment` of the corresponding `blobs`
//! - `proofs` - list of `KZGProof` of the corresponding `blobs` and `commitments`
//!
//! The node MUST validate `tx_payload_body` and verify the wrapped data against it. To do so, ensure that:
//!
//! - There are an equal number of `tx_payload_body.blob_versioned_hashes`, `blobs`, `commitments`, and `proofs`.
//! - The KZG `commitments` hash to the versioned hashes, i.e. `kzg_to_versioned_hash(commitments[i]) == tx_payload_body.blob_versioned_hashes[i]`
//! - The KZG `commitments` match the corresponding `blobs` and `proofs`. (Note: this can be optimized using `verify_blob_kzg_proof_batch`, with a proof for a
//!   random evaluation at a point derived from the commitment and blob data for each blob)
//!
//! For body retrieval responses (`BlockBodies`), the standard EIP-2718 blob transaction `TransactionPayload` is used.
//!
//! Nodes MUST NOT automatically broadcast blob transactions to their peers.
//! Instead, those transactions are only announced using `NewPooledTransactionHashes` messages, and can then be manually requested via `GetPooledTransactions`.
//!
//! ## Rationale
//!
//! ### On the path to sharding
//!
//! This EIP introduces blob transactions in the same format in which they are expected to exist in the final sharding specification.
//! This provides a temporary but significant scaling relief for rollups by allowing them to initially scale to 0.375 MB per slot,
//! with a separate fee market allowing fees to be very low while usage of this system is limited.
//!
//! The core goal of rollup scaling stopgaps is to provide temporary scaling relief,
//! without imposing extra development burdens on rollups to take advantage of this relief.
//! Today, rollups use calldata. In the future, rollups will have no choice but to use sharded data (also called "blobs")
//! because sharded data will be much cheaper.
//! Hence, rollups cannot avoid making a large upgrade to how they process data at least once along the way.
//! But what we _can_ do is ensure that rollups need to _only_ upgrade once.
//! This immediately implies that there are exactly two possibilities for a stopgap: (i) reducing the gas costs of existing calldata,
//! and (ii) bringing forward the format that will be used for sharded data, but not yet actually sharding it.
//! Previous EIPs were all a solution of category (i); this EIP is a solution of category (ii).
//!
//! The main tradeoff in designing this EIP is that of implementing more now versus having to implement more later:
//! do we implement 25% of the work on the way to full sharding, or 50%, or 75%?
//!
//! The work that is already done in this EIP includes:
//!
//! - A new transaction type, of the exact same format that will need to exist in "full sharding"
//! - _All_ of the execution-layer logic required for full sharding
//! - _All_ of the execution / consensus cross-verification logic required for full sharding
//! - Layer separation between `BeaconBlock` verification and data availability sampling blobs
//! - Most of the `BeaconBlock` logic required for full sharding
//! - A self-adjusting independent base fee for blobs
//!
//! The work that remains to be done to get to full sharding includes:
//!
//! - A low-degree extension of the `commitments` in the consensus layer to allow 2D sampling
//! - An actual implementation of data availability sampling
//! - PBS (proposer/builder separation), to avoid requiring individual validators to process 32 MB of data in one slot
//! - Proof of custody or similar in-protocol requirement for each validator to verify a particular part of the sharded data in each block
//!
//! This EIP also sets the stage for longer-term protocol cleanups. For example, its (cleaner) gas base fee update rule could be applied to the primary basefee calculation.
//!
//! ### How rollups would function
//!
//! Instead of putting rollup block data in transaction calldata, rollups would expect rollup block submitters
//! to put the data into blobs. This guarantees availability (which is what rollups need) but would be much cheaper than calldata.
//! Rollups need data to be available once, long enough to ensure honest actors can construct the rollup state, but not forever.
//!
//! Optimistic rollups only need to actually provide the underlying data when fraud proofs are being submitted.
//! The fraud proof can verify the transition in smaller steps, loading at most a few values of the blob at a time through calldata.
//! For each value it would provide a KZG proof and use the point evaluation precompile to verify the value against the versioned hash that was submitted before,
//! and then perform the fraud proof verification on that data as is done today.
//!
//! ZK rollups would provide two commitments to their transaction or state delta data:
//! the blob commitment (which the protocol ensures points to available data) and the ZK rollup's own commitment using whatever proof system the rollup uses internally.
//! They would use a proof of equivalence protocol, using the point evaluation precompile,
//! to prove that the two commitments refer to the same data.
//!
//! ### Versioned hashes & precompile return data
//!
//! We use versioned hashes (rather than commitments) as references to blobs in the execution layer to ensure forward compatibility with future changes.
//! For example, if we need to switch to Merkle trees + STARKs for quantum-safety reasons, then we would add a new version,
//! allowing the point evaluation precompile to work with the new format.
//! Rollups would not have to make any EVM-level changes to how they work;
//! sequencers would simply have to switch over to using a new transaction type at the appropriate time.
//!
//! However, the point evaluation happens inside a finite field, and it is only well defined if the field modulus is known. Smart contracts could contain a table mapping the commitment version to a modulus, but this would not allow smart contract to take into account future upgrades to a modulus that is not known yet. By allowing access to the modulus inside the EVM, the smart contract can be built so that it can use future commitments and proofs, without ever needing an upgrade.
//!
//! In the interest of not adding another precompile, we return the modulus and the polynomial degree directly from the point evaluation precompile. It can then be used by the caller. It is also "free" in that the caller can just ignore this part of the return value without incurring an extra cost -- systems that remain upgradable for the foreseeable future will likely use this route for now.
//!
//! ### Base fee per blob gas update rule
//!
//! The base fee per blob gas update rule is intended to approximate the formula `base_fee_per_blob_gas = MIN_BASE_FEE_PER_BLOB_GAS * e**(excess_blob_gas / BLOB_BASE_FEE_UPDATE_FRACTION)`,
//! where `excess_blob_gas` is the total "extra" amount of blob gas that the chain has consumed relative to the "targeted" number (`TARGET_BLOB_GAS_PER_BLOCK` per block).
//! Like EIP-1559, it's a self-correcting formula: as the excess goes higher, the `base_fee_per_blob_gas` increases exponentially, reducing usage and eventually forcing the excess back down.
//!
//! The block-by-block behavior is roughly as follows.
//! If block `N` consumes `X` blob gas, then in block `N+1` `excess_blob_gas` increases by `X - TARGET_BLOB_GAS_PER_BLOCK`,
//! and so the `base_fee_per_blob_gas` of block `N+1` increases by a factor of `e**((X - TARGET_BLOB_GAS_PER_BLOCK) / BLOB_BASE_FEE_UPDATE_FRACTION)`.
//! Hence, it has a similar effect to the existing EIP-1559, but is more "stable" in the sense that it responds in the same way to the same total usage regardless of how it's distributed.
//!
//! The parameter `BLOB_BASE_FEE_UPDATE_FRACTION` controls the maximum rate of change of the base fee per blob gas. It is chosen to target a maximum change rate of `e**(TARGET_BLOB_GAS_PER_BLOCK / BLOB_BASE_FEE_UPDATE_FRACTION) ≈ 1.125` per block.
//!
//! ### Throughput
//!
//! The values for `TARGET_BLOB_GAS_PER_BLOCK` and `MAX_BLOB_GAS_PER_BLOCK` are chosen to correspond to a target of 3 blobs (0.375 MB) and maximum of 6 blobs (0.75 MB) per block. These small initial limits are intended to minimize the strain on the network created by this EIP and are expected to be increased in future upgrades as the network demonstrates reliability under larger blocks.
//!
//! ## Backwards Compatibility
//!
//! ### Blob non-accessibility
//!
//! This EIP introduces a transaction type that has a distinct mempool version and execution-payload version,
//! with only one-way convertibility between the two. The blobs are in the network representation and not in the consensus representation;
//! instead, they are coupled with the beacon block. This means that there is now a part of a transaction that will not be accessible from the web3 API.
//!
//! ### Mempool issues
//!
//! Blob transactions have a large data size at the mempool layer, which poses a mempool `DoS` risk,
//! though not an unprecedented one as this also applies to transactions with large amounts of calldata.
//!
//! By only broadcasting announcements for blob transactions, receiving nodes will have control over which and how many transactions to receive,
//! allowing them to throttle throughput to an acceptable level.
//! [EIP-5793](./eip-5793.md) will give further fine-grained control to nodes by extending the `NewPooledTransactionHashes` announcement messages to include the transaction type and size.
//!
//! In addition, we recommend including a 1.1x base fee per blob gas bump requirement to the mempool transaction replacement rules.
//!
//! ## Test Cases
//!
//! Execution layer test cases for this EIP can be found in the [`eip4844_blobs`](https://github.com/ethereum/execution-spec-tests/tree/1983444bbe1a471886ef7c0e82253ffe2a4053e1/tests/cancun/eip4844_blobs) of the `ethereum/execution-spec-tests` repository. Consensus layer test cases can be found [here](https://github.com/ethereum/consensus-specs/tree/2297c09b7e457a13f7b2261a28cb45777be82f83/tests/core/pyspec/eth2spec/test/deneb).
//!
//! ## Security Considerations
//!
//! This EIP increases the bandwidth requirements per beacon block by a maximum of ~0.75 MB.
//! This is 40% larger than the theoretical maximum size of a block today (30M gas / 16 gas per calldata byte = 1.875M bytes), and so it will not greatly increase worst-case bandwidth.
//! Post-merge, block times are static rather than an unpredictable Poisson distribution, giving a guaranteed period of time for large blocks to propagate.
//!
//! The _sustained_ load of this EIP is much lower than alternatives that reduce calldata costs, even if the calldata is limited,
//! because there is no expectation that the blobs need to be stored for as long as an execution payload.
//! This makes it possible to implement a policy that these blobs must be kept for at least a certain period. The specific value chosen is `MIN_EPOCHS_FOR_BLOB_SIDECARS_REQUESTS` epochs, which is around 18 days,
//! a much shorter delay compared to proposed (but yet to be implemented) one-year rotation times for execution payload history.
//!
//!
//! Vitalik Buterin (@vbuterin), Dankrad Feist (@dankrad), Diederik Loerakker (@protolambda), George Kadianakis (@asn-d6), Matt Garnett (@lightclient), Mofi Taiwo (@Inphi), Ansgar Dietrichs (@adietrichs), "EIP-4844: Shard Blob Transactions," Ethereum Improvement Proposals, no. 4844, February 2022. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-4844>.

use asm::instruction::BlobHash;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-4844: Shard Blob Transactions.
pub struct Eip4844;

impl Eip for Eip4844 {
    const NUMBER: u32 = 4844;
}

introduces_instructions!(Eip4844, BlobHash);
