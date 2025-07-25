//! EIP-155: Simple replay attack protection.
//!
//! ### Hard fork
//! Spurious Dragon
//!
//! ### Parameters
//! - `FORK_BLKNUM`: 2,675,000
//! - `CHAIN_ID`: 1 (Mainnet)
//!
//! ### Specification
//!
//! If `block.number >= FORK_BLKNUM` and `CHAIN_ID` is available, then when computing the hash of a transaction for the purposes of signing, instead of hashing only six rlp encoded elements `(nonce, gasprice, startgas, to, value, data)`, you **SHOULD** hash nine rlp encoded elements `(nonce, gasprice, startgas, to, value, data, chainid, 0, 0)`.  If you do, then the `v` of the signature **MUST** be set to `{0,1} + CHAIN_ID * 2 + 35` where `{0,1}` is the parity of the `y` value of the curve point for which `r` is the x-value in the secp256k1 signing process.  If you choose to only hash 6 values, then `v` continues to be set to `{0,1} + 27` as previously.
//!
//! If `block.number >= FORK_BLKNUM` and `v = CHAIN_ID * 2 + 35` or `v = CHAIN_ID * 2 + 36`, then when computing the hash of a transaction for purposes of recovering, instead of hashing six rlp encoded elements `(nonce, gasprice, startgas, to, value, data)`, hash nine rlp encoded elements `(nonce, gasprice, startgas, to, value, data, chainid, 0, 0)`. The currently existing signature scheme using `v = 27` and `v = 28` remains valid and continues to operate under the same rules as it did previously.
//!
//! ### Example
//!
//! Consider a transaction with `nonce = 9`, `gasprice = 20 * 10**9`, `startgas = 21000`, `to = 0x3535353535353535353535353535353535353535`, `value = 10**18`, `data=''` (empty).
//!
//! The "signing data" becomes:
//!
//! ```python
//! 0xec098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a764000080018080
//! ```
//!
//! The "signing hash" becomes:
//!
//! ```python
//! 0xdaf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53
//! ```
//!
//! If the transaction is signed with the private key `0x4646464646464646464646464646464646464646464646464646464646464646`, then the v,r,s values become:
//!
//! ```python
//! (37, 18515461264373351373200002665853028612451056578545711640558177340181847433846, 46948507304638947509940763649030358759909902576025900602547168820602576006531)
//! ```
//!
//! Notice the use of 37 instead of 27. The signed tx would become:
//!
//! ```python
//! 0xf86c098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a76400008025a028ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276a067cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83
//! ```
//!
//! ### Rationale
//!
//! This would provide a way to send transactions that work on Ethereum without working on ETC or the Morden testnet. ETC is encouraged to adopt this EIP but replacing `CHAIN_ID` with a different value, and all future testnets, consortium chains and alt-etherea are encouraged to adopt this EIP replacing `CHAIN_ID` with a unique value.
//!
//!
//! ### List of Chain ID's:
//!
//! | `CHAIN_ID`     | Chain(s)                                   |
//! | ---------------| -------------------------------------------|
//! | 1              | Ethereum Mainnet                           |
//! | 2              | Morden (disused), Expanse Mainnet          |
//! | 3              | Ropsten                                    |
//! | 4              | Rinkeby                                    |
//! | 5              | Goerli                                     |
//! | 42             | Kovan                                      |
//! | 1337           | Geth private chains (default)              |
//!
//! Vitalik Buterin (@vbuterin), "EIP-155: Simple replay attack protection," Ethereum Improvement Proposals, no. 155, October 2016. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-155>.

use crate::eip::Eip;

/// EIP-155: Simple replay attack protection.
pub struct Eip155;

impl Eip for Eip155 {
    const NUMBER: u32 = 155;
}
