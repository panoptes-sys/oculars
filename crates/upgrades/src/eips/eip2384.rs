//! EIP-2384: Muir Glacier Difficulty Bomb Delay.
//!
//! ## Simple Summary
//! The average block times are increasing due to the difficulty bomb (also known as the "_ice age_") and slowly accelerating. This EIP proposes to delay the difficulty bomb for another 4,000,000 blocks (~611 days).
//!
//! ## Abstract
//! Starting with `MUIR_GLACIER_FORK_BLKNUM` the client will calculate the difficulty based on a fake block number suggesting to the client that the difficulty bomb is adjusting 9 million blocks later than the Homestead fork, which is also 7 million blocks later than the Byzantium fork and 4 million blocks later than the Constantinople fork.
//!
//! ## Motivation
//! The difficulty bomb started to become noticeable again on October 5th 2019 at block 8,600,000. Block times have been around 13.1s on average and now as of block 8,900,000 are around 14.3s. This will start to accelerate exponentially every 100,000 blocks. Estimating the added impact from the difficulty bomb on block times shows that we will see 20s block times near the end of December 2019 and 30s+ block times starting February 2020. This will start making the chain bloated and more costly to use. It's best to delay the difficulty bomb again to around the time of expected launch of the Eth2 finality gadget.
//!
//! ## Specification
//! #### Relax Difficulty with Fake Block Number
//! For the purposes of `calc_difficulty`, simply replace the use of `block.number`, as used in the exponential ice age component, with the formula:
//! ```python
//!     fake_block_number = max(0, block.number - 9_000_000) if block.number >= MUIR_GLACIER_FORK_BLKNUM else block.number
//! ```
//!
//! ## Rationale
//! This will delay the ice age by 52 million seconds (approximately 611 days), so the chain would be back at 20 second block times around July 2021. It's important to note this pushes the ice age 4,000,000 blocks from ~block 8,800,000 NOT from when this EIP is activated in a fork.
//!
//! ## Backwards Compatibility
//! This EIP is not forward compatible and introduces backwards incompatibilities in the difficulty calculation. Therefore, it should be included in a scheduled hardfork at a certain block number. It's suggested to include this EIP shortly after the Istanbul fork.
//!
//! Eric Conner (@econoar), "EIP-2384: Muir Glacier Difficulty Bomb Delay," Ethereum Improvement Proposals, no. 2384, November 2019. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-2384>.

use crate::eip::Eip;

/// EIP-2384: Muir Glacier Difficulty Bomb Delay.
pub struct Eip2384;

impl Eip for Eip2384 {
    const NUMBER: u32 = 2384;
}
