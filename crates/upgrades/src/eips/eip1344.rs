//! EIP-1344: `ChainID` opcode.
//! ## Abstract
//! This EIP adds an opcode that returns the current chain's EIP-155 unique identifier.
//!
//! ## Motivation
//! [EIP-155](./eip-155.md) proposes to use the chain ID to prevent replay attacks between different chains. It would be a great benefit to have the same possibility inside smart contracts when handling signatures, especially for Layer 2 signature schemes using [EIP-712](./eip-712.md).
//!
//! ## Specification
//! Adds a new opcode `CHAINID` at 0x46, which uses 0 stack arguments. It pushes the current chain ID onto the stack. Chain ID is a 256-bit value. The operation costs `G_base` to execute.
//!
//! The value of the current chain ID is obtained from the chain ID configuration, which should match the EIP-155 unique identifier a client will accept from incoming transactions. Please note that per EIP-155, it is not *required* that a transaction have an EIP-155 unique identifier, but in that scenario this opcode will still return the configured chain ID and not a default.
//!
//! ## Rationale
//! The current approach proposed by EIP-712 is to specify the chain ID at compile time. Using this approach will result in problems after a hardfork, as well as human error that may lead to loss of funds or replay attacks on signed messages.
//! By adding the proposed opcode it will be possible to access the current chain ID and validate signatures based on that.
//!
//! Currently, there is no specification for how chain ID is set for a particular network, relying on choices made manually by the client implementers and the chain community. There is a potential scenario where, during a "contentious split" over a divisive issue, a community using a particular value of chain ID will make a decision to split into two such chains. When this scenario occurs, it will be unsafe to maintain chain ID to the same value on both chains, as chain ID is used for replay protection for in-protocol transactions (per EIP-155), as well as for L2 and "meta-transaction" use cases (per EIP-712 as enabled by this proposal). There are two potential resolutions in this scenario under the current process: 1) one chain decides to modify their value of chain ID (while the other keeps it), or 2) both chains decide to modify their value of chain ID.
//!
//! In order to mitigate this situation, users of the proposed `CHAINID` opcode **must** ensure that their application can handle a potential update to the value of chain ID during their usage of their application in case this does occur, if required for the continued use of the application. A Trustless Oracle that logs the timestamp when a change is made to chain ID can be implemented either as an application-level feature inside the application contract system, or referenced as a globally standard contract. Failure to provide a mitigation for this scenario could lead to a sudden loss of legitimacy of previously signed off-chain messages, which could be an issue during settlement periods and other longer-term verification events for these types of messages. Not all applications of this opcode may need mitigations to handle this scenario, but developers should provide reasoning on a case-by-case basis.
//!
//! One example of a scenario where it would not make sense to leverage a global oracle is with the Plasma L2 paradigm. In the Plasma paradigm, an operator or group of operators submit blocks from the L2 network to the base chain (in this case Ethereum) summarizing transactions that have occurred on that chain. The submission of these blocks may not perfectly align with major events on the mainchain, such as a split causing an update of chain ID, which may cause a significant insecurity in the protocol if chain ID is utilized in signing messages. If the operators are not allowed to control the update of chain ID they will not be able to perfectly synchronize the update with their block submissions, and certain past transactions may be rejected because they do not align with the update. This is one example of the unintended consequences of trying to specify too much of the behavior of chain ID during a contentious split, and why having a simple opcode for access is most optimal, versus a more complicated precompile or contract.
//!
//! This proposed opcode would be the simplest possible way to implement this functionality, and allows developers the flexibility to implement their own global or local handling of chain ID changes, if required.
//!
//! ## Backwards Compatibility
//! This EIP is fully backwards compatible with all chains which implement EIP-155 chain ID domain separator for transaction signing.
//!
//! ## References
//! This was previously suggested as part of [EIP-901](https://github.com/ethereum/EIPs/issues/901).
//!
//! ## Test Cases
//! Test Cases added to [ethereum/tests](https://github.com/ethereum/tests/pull/627)
//!
//! Richard Meissner (@rmeissner), Bryant Eisenbach (@fubuloubu), "EIP-1344: `ChainID` opcode," Ethereum Improvement Proposals, no. 1344, August 2018. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-1344>.

use asm::instruction::ChainId;

use crate::eip::{Eip, macros::introduces_instructions};

/// EIP-1344: `ChainID` opcode.
pub struct Eip1344;

impl Eip for Eip1344 {
    const NUMBER: u32 = 1344;
}

introduces_instructions!(Eip1344, ChainId);
