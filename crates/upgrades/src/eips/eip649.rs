//! EIP-649: Metropolis Difficulty Bomb Delay and Block Reward Reduction.
//!
//! ## Simple Summary
//! The average block times are increasing due to the difficulty bomb (also known as the "_ice age_") slowly accelerating. This EIP proposes to delay the difficulty bomb for approximately one and a half year and to reduce the block rewards with the Byzantium fork, the first part of the Metropolis fork.
//!
//! ## Abstract
//! Starting with `BYZANTIUM_FORK_BLKNUM` the client will calculate the difficulty based on a fake block number suggesting the client that the difficulty bomb is adjusting around 3 million blocks later than previously specified with the Homestead fork. Furthermore, block rewards will be adjusted to a base of 3 ETH, uncle and nephew rewards will be adjusted accordingly.
//!
//! ## Motivation
//! The Casper development and switch to proof-of-stake is delayed, the Ethash proof-of-work should be feasible for miners and allow sealing new blocks every 15 seconds on average for another one and a half years. With the delay of the ice age, there is a desire to not suddenly also increase miner rewards. The difficulty bomb has been known about for a long time and now it's going to stop from happening. In order to maintain stability of the system, a block reward reduction that offsets the ice age delay would leave the system in the same general state as before. Reducing the reward also decreases the likelihood of a miner driven chain split as Ethereum approaches proof-of-stake.
//!
//! ## Specification
//! #### Relax Difficulty with Fake Block Number
//! For the purposes of `calc_difficulty`, simply replace the use of `block.number`, as used in the exponential ice age component, with the formula:
//!
//! ```python
//!     fake_block_number = max(0, block.number - 3_000_000) if block.number >= BYZANTIUM_FORK_BLKNUM else block.number
//! ```
//!
//! #### Adjust Block, Uncle, and Nephew rewards
//! To ensure a constant Ether issuance, adjust the block reward to `new_block_reward`, where
//! ```python
//!     new_block_reward = 3_000_000_000_000_000_000 if block.number >= BYZANTIUM_FORK_BLKNUM else block.reward
//! ```
//!
//! (3E18 wei, or 3,000,000,000,000,000,000 wei, or 3 ETH).
//!
//! Analogue, if an uncle is included in a block for `block.number >= BYZANTIUM_FORK_BLKNUM` such that `block.number - uncle.number = k`, the uncle reward is
//! ```python
//!     new_uncle_reward = (8 - k) * new_block_reward / 8
//! ```
//!
//! This is the existing pre-Metropolis formula for uncle rewards, simply adjusted with `new_block_reward`.
//!
//! The nephew reward for `block.number >= BYZANTIUM_FORK_BLKNUM` is
//! ```python
//!     new_nephew_reward = new_block_reward / 32
//! ```
//!
//! This is the existing pre-Metropolis formula for nephew rewards, simply adjusted with `new_block_reward`.
//!
//! ## Rationale
//! This will delay the ice age by 42 million seconds (approximately 1.4 years), so the chain would be back at 30 second block times at the end of 2018. An alternate proposal was to add special rules to the difficulty calculation to effectively _pause_ the difficulty between different blocks. This would lead to similar results.
//!
//! This was previously discussed at All Core Devs Meeting [#09](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%209.md#metropolis-timing-and-roadmap-discussion), [#12](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2012.md#5-metropolis-update), [#13](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2013.md#3-eip-186-reduce-eth-issuance-before-proof-of-stake-hudson), and [#14](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2014.md#1-eip-186-reduce-eth-issuance-before-proof-of-stake-core-devs). Consensus on the specification was achieved in All Core Devs Meeting [#19](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2019.md) and specification drafted in EIP issue [#649](https://github.com/ethereum/EIPs/issues/649). It was decided to replace EIP [#186](https://github.com/ethereum/EIPs/issues/186) and include the block reward reduction along with the difficulty bomb delay in All Core Devs Meeting [#20](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2020.md) and [#21](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2021.md); accepted in [#22](https://github.com/ethereum/pm/blob/master/AllCoreDevs-EL-Meetings/Meeting%2022.md).
//!
//! ## Backwards Compatibility
//! This EIP is not forward compatible and introduces backwards incompatibilities in the difficulty calculation, as well as the block, uncle and nephew reward structure. Therefore, it should be included in a scheduled hardfork at a certain block number. It's suggested to include this EIP in the first of the two Metropolis hard-forks, the _Byzantium_ fork.
//!
//! ## Test Cases
//! Test cases exist in ethereum/tests [#269](https://github.com/ethereum/tests/pull/269).
//!
//! Afri Schoedon (@5chdn), Vitalik Buterin (@vbuterin), "EIP-649: Metropolis Difficulty Bomb Delay and Block Reward Reduction," Ethereum Improvement Proposals, no. 649, June 2017. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-649>.

use crate::eip::Eip;

/// EIP-649: Metropolis Difficulty Bomb Delay and Block Reward Reduction.
pub struct Eip649;

impl Eip for Eip649 {
    const NUMBER: u32 = 649;
}
