//! EIP-3675: Upgrade consensus to Proof-of-Stake.
//!
//! ## Abstract
//!
//! This EIP deprecates Proof-of-Work (`PoW`) and supersedes it with the new Proof-of-Stake consensus mechanism (`PoS`) driven by the beacon chain. Information on the bootstrapping of the new consensus mechanism is documented in [EIP-2982](./eip-2982.md). Full specification of the beacon chain can be found in the `ethereum/consensus-specs` repository.
//!
//! This document specifies the set of changes to the block structure, block processing, fork choice rule and network interface introduced by the consensus upgrade.
//!
//!
//! ## Motivation
//!
//! The beacon chain network has been up and running since December 2020. Neither safety nor liveness failures were detected during this period of time. This long period of running without failure demonstrates the sustainability of the beacon chain system and its readiness to become a security provider for the Ethereum Mainnet.
//!
//! To understand the motivation of introducing the Proof-of-Stake consensus see the Motivation section of [EIP-2982](./eip-2982.md#motivation).
//!
//!
//! ## Specification
//!
//! ### Definitions
//!
//! * **`PoW` block**: Block that is built and verified by the existing proof-of-work mechanism. In other words, a block of the Ethereum network before the consensus upgrade.
//! * **`PoS` block**: Block that is built and verified by the new proof-of-stake mechanism.
//! * **Terminal `PoW` block**: A `PoW` block that satisfies the following conditions --
//!   `pow_block.total_difficulty >= TERMINAL_TOTAL_DIFFICULTY` *and* `pow_block.parent_block.total_difficulty < TERMINAL_TOTAL_DIFFICULTY`.
//!   There can be more than one terminal `PoW` block in the network, e.g. multiple children of the same pre-terminal block.
//! * **`TERMINAL_TOTAL_DIFFICULTY`** The amount of total difficulty reached by the network that triggers the consensus upgrade. Ethereum Mainnet configuration **MUST** have this parameter set to the value `58750000000000000000000`.
//! * **`TRANSITION_BLOCK`** The earliest `PoS` block of the canonical chain, i.e. the `PoS` block with the lowest block height.
//! * **`POS_FORKCHOICE_UPDATED`** An event occurring when the state of the proof-of-stake fork choice is updated.
//! * **`FORK_NEXT_VALUE`** A block number set to the `FORK_NEXT` parameter for the upcoming consensus upgrade.
//! * **`TERMINAL_BLOCK_HASH`** Designates the hash of the terminal `PoW` block if set, i.e. if not stubbed with `0x0000000000000000000000000000000000000000000000000000000000000000`.
//! * **`TERMINAL_BLOCK_NUMBER`** Designates the number of the terminal `PoW` block if `TERMINAL_BLOCK_HASH` is set.
//!
//! #### `PoS` events
//!
//! Events having the `POS_` prefix in the name (`PoS` events) are emitted by the new proof-of-stake consensus mechanism. They signify the corresponding assertion that has been made regarding a block specified by the event. The underlying logic of `PoS` events can be found in the beacon chain specification. On the occurrence of each `PoS` event the corresponding action that is specified by this EIP **MUST** be taken.
//!
//! The details provided below must be taken into account when reading those parts of the specification that refer to the `PoS` events:
//! * Reference to a block that is contained by `PoS` events is provided in a form of a block hash unless another is explicitly specified.
//! * A `POS_FORKCHOICE_UPDATED` event contains references to the head of the canonical chain and to the most recent finalized block. Before the first finalized block occurs in the system the finalized block hash provided by this event is stubbed with `0x0000000000000000000000000000000000000000000000000000000000000000`.
//! * **`FIRST_FINALIZED_BLOCK`** The first finalized block that is designated by `POS_FORKCHOICE_UPDATED` event and has the hash that differs from the stub.
//!
//!
//! ### Client software configuration
//!
//! The following set of parameters is a part of client software configuration and **MUST** be included into its binary distribution:
//! * `TERMINAL_TOTAL_DIFFICULTY`
//! * `FORK_NEXT_VALUE`
//! * `TERMINAL_BLOCK_HASH`
//! * `TERMINAL_BLOCK_NUMBER`
//!
//! *Note*: If `TERMINAL_BLOCK_HASH` is stubbed with `0x0000000000000000000000000000000000000000000000000000000000000000` then `TERMINAL_BLOCK_HASH` and `TERMINAL_BLOCK_NUMBER` parameters **MUST NOT** take an effect.
//!
//!
//! ### `PoW` block processing
//!
//! `PoW` blocks that are descendants of any terminal `PoW` block **MUST NOT** be imported. This implies that a terminal `PoW` block will be the last `PoW` block in the canonical chain.
//!
//!
//! ### Constants
//!
//! | Name | Value |
//! |-|-|
//! | **`MAX_EXTRA_DATA_BYTES`** | `32` |
//!
//! ### Block structure
//!
//! Beginning with `TRANSITION_BLOCK`, a number of previously dynamic block fields are deprecated by enforcing these values to instead be constants. Each block field listed in the table below **MUST** be replaced with the corresponding constant value.
//!
//! | Field | Constant value | Comment |
//! |-|-|-|
//! | **`ommersHash`** | `0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347` | `= Keccak256(RLP([]))` |
//! | **`difficulty`** | `0` |  |
//! | **`mixHash`** | `0x0000000000000000000000000000000000000000000000000000000000000000` |  |
//! | **`nonce`** | `0x0000000000000000` |  |
//! | **`ommers`** | `[]` | `RLP([]) = 0xc0`  |
//!
//! Beginning with `TRANSITION_BLOCK`, the validation of the block's **`extraData`** field changes: The length of the block's **`extraData`** **MUST** be less than or equal to **`MAX_EXTRA_DATA_BYTES`** bytes.
//!
//! *Note*: Logic and validity conditions of block fields that are *not* specified here **MUST** remain unchanged. Additionally, the overall block format **MUST** remain unchanged.
//!
//! *Note*: Subsequent EIPs may override the constant values specified above to provide additional functionality. For an example, see [EIP-4399](./eip-4399.md).
//!
//!
//! ### Block validity
//!
//! Beginning with `TRANSITION_BLOCK`, the block validity conditions **MUST** be altered by the following:
//! * Remove verification of the block's **`difficulty`** value with respect to the difficulty formula.
//! * Remove verification of the block's **`nonce`** and **`mixHash`** values with respect to the Ethash function.
//! * Remove all validation rules that are evaluated over the list of ommers and each member of this list.
//! * Add verification of the fields noted in the [block structure](#block-structure) section.
//!
//! *Note*: If one of the new rules fails then the block **MUST** be invalidated.
//!
//! *Note*: Validity rules that are not specified in the list above **MUST** remain unchanged.
//!
//! #### Transition block validity
//!
//! In addition to satisfying the above conditions, `TRANSITION_BLOCK` **MUST** be a child of a terminal `PoW` block. That is, a parent of `TRANSITION_BLOCK` **MUST** satisfy terminal `PoW` block conditions.
//!
//!
//! ### Block and ommer rewards
//!
//! Beginning with `TRANSITION_BLOCK`, block and ommer rewards are deprecated. Specifically, the following actions **MUST** be taken:
//! * Remove increasing the balance of the block's **`beneficiary`** account by the block reward.
//! * Remove increasing the balance of the block's **`beneficiary`** account by the ommer inclusion reward per each ommer.
//! * Remove increasing the balance of the ommer's **`beneficiary`** account by the ommer block reward per each ommer.
//!
//! *Note*: Transaction fee mechanics affecting the block's `beneficiary` account **MUST** remain unchanged.
//!
//!
//! ### Fork choice rule
//!
//! If set, `TERMINAL_BLOCK_HASH` parameter affects the `PoW` heaviest chain rule in the following way:
//! * Canonical blockchain **MUST** contain a block with the hash defined by `TERMINAL_BLOCK_HASH` parameter at the height defined by `TERMINAL_BLOCK_NUMBER` parameter.
//!
//! *Note*: This rule is akin to block hash whitelisting functionality already present in client software implementations.
//!
//! As of the first `POS_FORKCHOICE_UPDATED` event, the fork choice rule **MUST** be altered in the following way:
//! * Remove the existing `PoW` heaviest chain rule.
//! * Adhere to the new `PoS` LMD-GHOST rule.
//!
//! The new `PoS` LMD-GHOST fork choice rule is specified as follows. On each occurrence of a `POS_FORKCHOICE_UPDATED` event including the first one, the following actions **MUST** be taken:
//! * Consider the chain starting at genesis and ending with the head block nominated by the event as the canonical blockchain.
//! * Set the head of the canonical blockchain to the corresponding block nominated by the event.
//! * Beginning with the `FIRST_FINALIZED_BLOCK`, set the most recent finalized block to the corresponding block nominated by the event.
//!
//! Changes to the block tree store that are related to the above actions **MUST** be applied atomically.
//!
//! *Note*: This rule **MUST** be strictly enforced. "Optimistic" updates to the head **MUST NOT** be made. That is -- if a new block is processed on top of the current head block, this new block becomes the new head if and only if an accompanying `POS_FORKCHOICE_UPDATED` event occurs.
//!
//! ### Network
//!
//! #### Fork identifier
//!
//! For the purposes of the [EIP-2124](./eip-2124.md) fork identifier, nodes implementing this EIP **MUST** set the `FORK_NEXT` parameter to the `FORK_NEXT_VALUE`.
//!
//! #### devp2p
//!
//! The networking stack **SHOULD NOT** send the following messages if they advertise the descendant of any terminal `PoW` block:
//! * `NewBlockHashes (0x01)`
//! * `NewBlock (0x07)`
//!
//! Beginning with receiving the `FIRST_FINALIZED_BLOCK`, the networking stack **MUST** discard the following ingress messages:
//! * `NewBlockHashes (0x01)`
//! * `NewBlock (0x07)`
//!
//! Beginning with receiving the finalized block next to the `FIRST_FINALIZED_BLOCK`, the networking stack **MUST** remove the handlers corresponding to the following messages:
//! * `NewBlockHashes (0x01)`
//! * `NewBlock (0x07)`
//!
//! Peers that keep sending these messages after the handlers have been removed **SHOULD** be disconnected.
//!
//! *Note:* The logic of message handlers that are not affected by this section **MUST** remain unchanged.
//!
//!
//! ## Rationale
//!
//! The changes specified in this EIP target a minimal requisite set of consensus and client software modifications to safely replace the existing proof-of-work consensus algorithm with the new proof-of-stake consensus represented by the already in-production beacon chain.
//!
//! This EIP was designed to minimize the complexity of hot-swapping the live consensus of the Ethereum network. Both the safety of the operation and time to production were taken into consideration. Additionally, a minimal changeset helps ensure that *most* smart contracts and services will continue to function as intended during and after the transition with little to no required intervention.
//!
//! ### Total difficulty triggering the upgrade
//!
//! See [Security considerations](#terminal-total-difficulty-vs-block-number).
//!
//! ### Parameterizing terminal block hash
//!
//! See [Security considerations](#terminal-pow-block-overriding).
//!
//! ### Halting the import of `PoW` blocks
//!
//! See [Security considerations](#halt-the-importing-of-pow-blocks).
//!
//! ### Replacing block fields with constants
//!
//! Deprecated block fields are replaced with constant values to ensure the block format remains backwards compatible. Preserving the block format aids existing smart contracts and services in providing uninterrupted service during and after this transition.
//!
//! Particularly, this is important for those smart contracts that verify Merkle proofs of transaction/receipt inclusion and state by validating the hash of externally provided block header against the corresponding value returned by the `BLOCKHASH` operation.
//!
//! This change introduces an additional validity rule that enforces the replacement of deprecated block fields.
//!
//! ### Replacing `difficulty` with `0`
//!
//! After deprecating the proof-of-work the notion of difficulty no longer exists and replacing the block header **`difficulty`** field with `0` constant is semantically sound.
//!
//! ### Changing block validity rules
//!
//! The rule set enforcing the `PoW` seal validity is replaced with the corresponding `PoS` rules along with the consensus upgrade as the rationale behind this change.
//!
//! An additional rule validating a set of deprecated block fields is required by the block format changes introduced by this specification.
//!
//! ### Removing block rewards
//!
//! Existing rewards for producing and sealing blocks are deprecated along with the `PoW` mechanism. The new `PoS` consensus becomes both responsible for sealing blocks and for issuing block rewards once this specification enters into effect.
//!
//! ### Supplanting fork choice rule
//!
//! The fork choice rule of the `PoW` mechanism becomes completely irrelevant after the upgrade and is replaced with the corresponding rule of the new `PoS` consensus mechanism.
//!
//! ### Remove of `POS_CONSENSUS_VALIDATED`
//!
//! In prior draft versions of this EIP, an additional POS event -- `POS_CONSENSUS_VALIDATED` -- was required as a validation condition for blocks. This event gave the signal to either fully incorporate or prune the block from the block tree.
//!
//! This event was removed for two reasons:
//! 1. This event was an unnecessary optimization to allow for pruning of "bad" blocks from the block tree. This optimization was unnecessary because the `PoS` consensus would never send `POS_FORKCHOICE_UPDATED` for any such bad blocks or their descendants, and eventually any such blocks would be able to be pruned after a `PoS` finality event of an alternative branch in the block tree.
//! 2. This event was dangerous in some scenarios because a block could be referenced by two *different* and conflicting `PoS` branches. Thus for the same block in some scenarios, both a `POS_CONSENSUS_VALIDATED == TRUE` and `POS_CONSENSUS_VALIDATED == FALSE` event could sent, entirely negating the ability to safely perform the optimization in (1).
//!
//! ### EIP-2124 fork identifier
//!
//! The value of `FORK_NEXT` in EIP-2124 refers to the block number of the next fork a given node knows about and `0` otherwise.
//!
//! The number of `TRANSITION_BLOCK` cannot be known ahead of time given the dynamic nature of the transition trigger condition. As the block will not be known a priori, nodes can't use its number for `FORK_NEXT` and in light of this fact an explicitly set `FORK_NEXT_VALUE` is used instead.
//!
//! ### Removing block gossip
//!
//! After the upgrade of the consensus mechanism only the beacon chain network will have enough information to validate a block. Thus, block gossip provided by the `eth` network protocol will become unsafe and is deprecated in favour of the block gossip existing in the beacon chain network.
//!
//! It is recommended for the client software to not propagate descendants of any terminal `PoW` block to reduce the load on processing the P2P component and stop operating in the environment with unknown security properties.
//!
//! ### Restricting the length of `extraData`
//!
//! The `extraData` field is defined as a maximum of `32` bytes in the yellow paper. Thus mainnet and most `PoW` testnets cap the value at `32` bytes.  `extraData` fields of greater length are used by clique testnets and other networks to carry special signature/consensus schemes. This EIP restricts the length of `extraData` to `32` bytes because any network that is transitioning from another consensus mechanism to a beacon chain `PoS` consensus mechanism no longer needs extended or unbounded `extraData`.
//!
//! ## Backwards Compatibility
//!
//! This EIP introduces backward incompatibilities in block validity, block rewards and fork choice rule.
//!
//! The design of the consensus upgrade specified by this document does not introduce backward incompatibilities for existing applications and services built on top of Ethereum except for those that are described in the [EVM](#evm) section below or heavily depends on the `PoW` consensus in any other way.
//!
//!
//! ### EVM
//!
//! Although this EIP does not introduce any explicit changes to the EVM there are a couple of places where it may affect the logic of existing smart contracts.
//!
//! #### DIFFICULTY
//!
//! `DIFFICULTY` operation will always return `0` after this EIP takes effect and deprecates the **`difficulty`** field by replacing it with `0` constant.
//!
//! *Note:* Altering the `DIFFICULTY` semantics to return randomness accumulated by the beacon chain is under consideration but will be introduced in a separate EIP.
//!
//! #### BLOCKHASH
//!
//! Pseudo-random numbers obtained as the output of `BLOCKHASH` operation become more insecure after this EIP takes effect and the `PoW` mechanism (which decreases the malleability of block hashes) gets supplanted by `PoS`.
//!
//!
//! ## Test Cases
//!
//! * Block validity
//!     * Beginning with `TRANSITION_BLOCK`, block is invalidated if any of the following is true:
//!         * `ommersHash != Keccak256(RLP([]))`
//!         * `difficulty != 0`
//!         * `nonce != 0x0000000000000000`
//!         * `len(extraData) > MAX_EXTRA_DATA_BYTES`
//!   * Beginning with `TRANSITION_BLOCK`, block rewards aren't added to `beneficiary` account
//! * Client software adheres to `PoS` LMD-GHOST rule
//!   * Head and finalized blocks are set according to the recent `POS_FORKCHOICE_UPDATED` event
//!   * No fork choice state is updated unless `POS_FORKCHOICE_UPDATED` event is received
//! * Transition process
//!   * Client software doesn't process any `PoW` block beyond a terminal `PoW` block
//!   * Beginning with `TRANSITION_BLOCK`, client software applies new block validity rules
//!   * Beginning with the first `POS_FORKCHOICE_UPDATED`, client software switches its fork choice rule to `PoS` LMD-GHOST
//!   * `TRANSITION_BLOCK` must be a child of a terminal `PoW` block
//!   * `NewBlockHashes (0x01)` and `NewBlock (0x07)` network messages are discarded after receiving the `FIRST_FINALIZED_BLOCK`
//!
//!
//! ## Security Considerations
//!
//! ### Beacon chain
//!
//! See Security Considerations section of [EIP-2982](./eip-2982.md#security-considerations).
//!
//! ### Transition process
//!
//! The transition process used to take this specification into effect is a more sophisticated version of a hardfork -- the regular procedure of applying backwards incompatible changes in the Ethereum network. This process has multiple successive steps instead of the normal block-height point condition of simpler hardforks.
//!
//! The complexity of this upgrade process stems from this fork targeting the underlying consensus mechanism rather than the execution layer within the consensus mechanism. Although the design seeks simplicity where possible, safety and liveness considerations during this transition have been prioritized.
//!
//! #### Terminal total difficulty vs block number
//!
//! Using a pre-defined block number for the hardfork is unsafe in this context due to the `PoS` fork choice taking priority during the transition.
//!
//! An attacker may use a minority of hash power to build a malicious chain fork that would satisfy the block height requirement. Then the first `PoS` block may be maliciously proposed on top of the `PoW` block from this adversarial fork, becoming the head and subverting the security of the transition.
//!
//! To protect the network from this attack scenario, difficulty accumulated by the chain (total difficulty) is used to trigger the upgrade.
//!
//! #### Ability to jump between terminal `PoW` blocks
//!
//! There could be the case when a terminal `PoW` block is not observed by the majority of network participants due to (temporal) network partitioning. In such a case, this minority would switch their fork choice to the new rule provided by the `PoS` rooted on the minority terminal `PoW` block that they observed.
//!
//! The transition process allows the network to re-org between forks with different terminal `PoW` blocks as long as (a) these blocks satisfy the terminal `PoW` block conditions and (b) the `FIRST_FINALIZED_BLOCK` has not yet been received. This provides resilience against adverse network conditions during the transition process and prevents irreparable forks/partitions.
//!
//! #### Halt the importing of `PoW` blocks
//!
//! Suppose the part of the client software that is connected to the beacon chain network goes offline before the Ethereum network reaches the `TERMINAL_TOTAL_DIFFICULTY` and stays offline while the network meets this threshold. Such an event makes the client software unable to switch to `PoS` and allows it to keep following the `PoW` chain if this chain is being built beyond the terminal `PoW` block. Depending on how long the beacon chain part was offline, it could result in different adverse effects such as:
//! * The client has no post-state for the terminal `PoW` block (the state has been pruned) which prevents it from doing the re-org to the `PoS` chain and leaving syncing from scratch as the only option to recover.
//! * An application, a user or a service uses the data from the wrong fork (`PoW` chain that is kept being built) which can cause security issues on their side.
//!
//! Not importing `PoW` blocks that are beyond the terminal `PoW` block prevents these adverse effects on safety/re-orgs in the event of software or configuration failures *in favor* of a liveness failure.
//!
//! #### Terminal `PoW` block overriding
//!
//! There is a mechanism allowing for accelerating the consensus upgrade in emergency cases.
//! This EIP considers the following emergency case scenarios for the acceleration to come into effect:
//! * A drop of the network hashing rate which delays the upgrade significantly.
//! * Attacks on the `PoW` network before the upgrade.
//!
//! The first case can be safely accelerated by updating the following parameters:
//! * `TERMINAL_TOTAL_DIFFICULTY` -- reset to a value that is closer in time than the original one.
//! * `FORK_NEXT_VALUE` -- adjust accordingly.
//!
//! The second, more dire attack scenario requires a more invasive override:
//! * `TERMINAL_BLOCK_HASH` -- set to the hash of a certain block to become the terminal `PoW` block.
//! * `TERMINAL_BLOCK_NUMBER` -- set to the number of a block designated by `TERMINAL_BLOCK_HASH`.
//! * `TERMINAL_TOTAL_DIFFICULTY` -- set to the total difficulty value of a block designated by `TERMINAL_BLOCK_HASH`.
//! * `FORK_NEXT_VALUE` -- adjust accordingly.
//!
//! *Note*: Acceleration in the second case is considered for the most extreme of scenarios because it will result in a non-trivial liveness failure on Ethereum Mainnet.
//!
//! ### Ancient blocks are no longer a requisite for a network security
//!
//! Keeping historical blocks starting from genesis is essential in the `PoW` network. A header of every block that belongs to a particular chain is required to justify the validity of this chain with respect to the `PoW` seal.
//!
//! Validating the entire history of the chain is not required by the new `PoS` mechanism. Instead, the sync process in the `PoS` network relies on weak subjectivity checkpoints, which are historical snapshots shared by peers on the network. This means historical blocks beyond weak subjectivity checkpoint are no longer a requisite for determining the canonical blockchain.
//!
//! Specification of weak subjectivity checkpoints can be found in the `ethereum/consensus-specs` repository.
//!
//! Mikhail Kalinin (@mkalinin), Danny Ryan (@djrtwo), Vitalik Buterin (@vbuterin), "EIP-3675: Upgrade consensus to Proof-of-Stake," Ethereum Improvement Proposals, no. 3675, July 2021. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-3675>.

use crate::eip::Eip;

/// EIP-3675: Upgrade consensus to Proof-of-Stake.
pub struct Eip3675;

impl Eip for Eip3675 {
    const NUMBER: u32 = 3675;
}
