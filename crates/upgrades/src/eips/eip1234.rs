//! EIP-1234: Constantinople Difficulty Bomb Delay and Block Reward Adjustment.
//!
//! ## Simple Summary
//! The average block times are increasing due to the difficulty bomb (also known as the "_ice age_") slowly accelerating. This EIP proposes to delay the difficulty bomb for approximately 12 months and to reduce the block rewards with the Constantinople fork, the second part of the Metropolis fork.
//!
//! ## Abstract
//! Starting with `CNSTNTNPL_FORK_BLKNUM` the client will calculate the difficulty based on a fake block number suggesting the client that the difficulty bomb is adjusting around 5 million blocks later than previously specified with the Homestead fork. Furthermore, block rewards will be adjusted to a base of 2 ETH, uncle and nephew rewards will be adjusted accordingly.
//!
//! ## Motivation
//! The Casper development and switch to proof-of-stake is delayed, the Ethash proof-of-work should be feasible for miners and allow sealing new blocks every 15 seconds on average for another 12 months. With the delay of the ice age, there is a desire to not suddenly also increase miner rewards. The difficulty bomb has been known about for a long time and now it's going to stop from happening. In order to maintain stability of the system, a block reward reduction that offsets the ice age delay would leave the system in the same general state as before. Reducing the reward also decreases the likelihood of a miner driven chain split as Ethereum approaches proof-of-stake.
//!
//! ## Specification
//! #### Relax Difficulty with Fake Block Number
//! For the purposes of `calc_difficulty`, simply replace the use of `block.number`, as used in the exponential ice age component, with the formula:
//! ```python
//!     fake_block_number = max(0, block.number - 5_000_000) if block.number >= CNSTNTNPL_FORK_BLKNUM else block.number
//! ```
//! #### Adjust Block, Uncle, and Nephew rewards
//! To ensure a constant Ether issuance, adjust the block reward to `new_block_reward`, where
//! ```python
//!     new_block_reward = 2_000_000_000_000_000_000 if block.number >= CNSTNTNPL_FORK_BLKNUM else block.reward
//! ```
//! (2E18 wei, or 2,000,000,000,000,000,000 wei, or 2 ETH).
//!
//! Analogue, if an uncle is included in a block for `block.number >= CNSTNTNPL_FORK_BLKNUM` such that `block.number - uncle.number = k`, the uncle reward is
//! ```python
//!     new_uncle_reward = (8 - k) * new_block_reward / 8
//! ```
//! This is the existing pre-Constantinople formula for uncle rewards, simply adjusted with `new_block_reward`.
//!
//! The nephew reward for `block.number >= CNSTNTNPL_FORK_BLKNUM` is
//! ```python
//!     new_nephew_reward = new_block_reward / 32
//! ```
//! This is the existing pre-Constantinople formula for nephew rewards, simply adjusted with `new_block_reward`.
//!
//! ## Rationale
//! This will delay the ice age by 29 million seconds (approximately 12 months), so the chain would be back at 30 second block times in winter 2019. An alternate proposal was to add special rules to the difficulty calculation to effectively _pause_ the difficulty between different blocks. This would lead to similar results.
//!
//! This was previously discussed at All Core Devs Meeting [#42](https://github.com/ethereum/pm/blob/6dbd82303bfcb697eaf9a76de37f5fa570e6379d/AllCoreDevs-EL-Meetings/Meeting%2042.md) and subsequent meetings; and accepted in the Constantinople Session [#1](https://github.com/ethereum/pm/issues/55).
//!
//! ## Backwards Compatibility
//! This EIP is not forward compatible and introduces backwards incompatibilities in the difficulty calculation, as well as the block, uncle and nephew reward structure. Therefore, it should be included in a scheduled hardfork at a certain block number. It's suggested to include this EIP in the second Metropolis hard-fork, _Constantinople_.
//!
//! Afri Schoedon (@5chdn), "EIP-1234: Constantinople Difficulty Bomb Delay and Block Reward Adjustment," Ethereum Improvement Proposals, no. 1234, July 2018. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-1234>.

use crate::eip::Eip;

/// EIP-1234: Constantinople Difficulty Bomb Delay and Block Reward Adjustment.
pub struct Eip1234;

impl Eip for Eip1234 {
    const NUMBER: u32 = 1234;
}
