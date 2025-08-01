//! EIP-150: Gas cost changes for IO-heavy operations.
//!
//! ### Meta reference
//!
//! [Tangerine Whistle](./eip-608.md).
//!
//! ### Parameters
//!
//! |   `FORK_BLKNUM`   |  `CHAIN_ID`  | `CHAIN_NAME`  |
//! |-------------------|--------------|---------------|
//! |     2,463,000     |      1       |    Mainnet    |
//!
//! ### Specification
//!
//! If `block.number >= FORK_BLKNUM`, then:
//! - Increase the gas cost of EXTCODESIZE to 700 (from 20).
//! - Increase the base gas cost of EXTCODECOPY to 700 (from 20).
//! - Increase the gas cost of BALANCE to 400 (from 20).
//! - Increase the gas cost of SLOAD to 200 (from 50).
//! - Increase the gas cost of CALL, DELEGATECALL, CALLCODE to 700 (from 40).
//! - Increase the gas cost of SELFDESTRUCT to 5000 (from 0).
//! - If SELFDESTRUCT hits a newly created account, it triggers an additional gas cost of 25000 (similar to CALLs).
//! - Increase the recommended gas limit target to 5.5 million.
//! - Define "all but one 64th" of `N` as `N - floor(N / 64)`.
//! - If a call asks for more gas than the maximum allowed amount (i.e. the total amount of gas remaining in the parent after subtracting the gas cost of the call and memory expansion), do not return an OOG error; instead, if a call asks for more gas than all but one 64th of the maximum allowed amount, call with all but one 64th of the maximum allowed amount of gas (this is equivalent to a version of EIP-90<sup>[1](https://github.com/ethereum/EIPs/issues/90)</sup> plus EIP-114<sup>[2](https://github.com/ethereum/EIPs/issues/114)</sup>). CREATE only provides all but one 64th of the parent gas to the child call.
//!
//! That is, substitute:
//!
//! ```python
//!         extra_gas = (not ext.account_exists(to)) * opcodes.GCALLNEWACCOUNT + \
//!             (value > 0) * opcodes.GCALLVALUETRANSFER
//!         if compustate.gas < gas + extra_gas:
//!             return vm_exception('OUT OF GAS', needed=gas+extra_gas)
//!         submsg_gas = gas + opcodes.GSTIPEND * (value > 0)
//! ```
//!
//! With:
//!
//! ```python
//!         def max_call_gas(gas):
//!           return gas - (gas // 64)
//!
//!         extra_gas = (not ext.account_exists(to)) * opcodes.GCALLNEWACCOUNT + \
//!             (value > 0) * opcodes.GCALLVALUETRANSFER
//!         if compustate.gas < extra_gas:
//!             return vm_exception('OUT OF GAS', needed=extra_gas)
//!         if compustate.gas < gas + extra_gas:
//!             gas = min(gas, max_call_gas(compustate.gas - extra_gas))
//!         submsg_gas = gas + opcodes.GSTIPEND * (value > 0)
//! ```
//!
//! ### Rationale
//!
//! Recent denial-of-service attacks have shown that opcodes that read the state tree are under-priced relative to other opcodes. There are software changes that have been made, are being made and can be made in order to mitigate the situation; however, the fact will remain that such opcodes will be by a substantial margin the easiest known mechanism to degrade network performance via transaction spam. The concern arises because it takes a long time to read from disk, and is additionally a risk to future sharding proposals as the "attack transactions" that have so far been most successful in degrading network performance would also require tens of megabytes to provide Merkle proofs for. This EIP increases the cost of storage reading opcodes to address this concern. The costs have been derived from an updated version of the calculation table used to generate the 1.0 gas costs: <https://docs.google.com/spreadsheets/d/15wghZr-Z6sRSMdmRmhls9dVXTOpxKy8Y64oy9MvDZEQ/edit#gid=0>; the rules attempt to target a limit of 8 MB of data that needs to be read in order to process a block, and include an estimate of 500 bytes for a Merkle proof for SLOAD and 1000 for an account.
//!
//! This EIP aims to be simple, and adds a flat penalty of 300 gas on top of the costs calculated in this table to account for the cost of loading the code (~17–21 kb in the worst case).
//!
//! The EIP 90 gas mechanic is introduced because without it, all current contracts that make calls would stop working as they use an expression like `msg.gas - 40` to determine how much gas to make a call with, relying on the gas cost of calls being 40. Additionally, EIP 114 is introduced because, given that we are making the cost of a call higher and less predictable, we have an opportunity to do it at no extra cost to currently available guarantees, and so we also achieve the benefit of replacing the call stack depth limit with a "softer" gas-based restriction, thereby eliminating call stack depth attacks as a class of attack that contract developers have to worry about and hence increasing contract programming safety. Note that with the given parameters, the de-facto maximum call stack depth is limited to ~340 (down from ~1024), mitigating the harm caused by any further potential quadratic-complexity `DoS` attacks that rely on calls.
//!
//! The gas limit increase is recommended so as to preserve the de-facto transactions-per-second processing capability of the system for average contracts.
//!
//! Vitalik Buterin (@vbuterin), "EIP-150: Gas cost changes for IO-heavy operations," Ethereum Improvement Proposals, no. 150, September 2016. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-150>.

use crate::eip::Eip;

/// EIP-150: Gas cost changes for IO-heavy operations.
pub struct Eip150;

impl Eip for Eip150 {
    const NUMBER: u32 = 150;
}
