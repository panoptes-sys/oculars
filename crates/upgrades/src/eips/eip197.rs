//! EIP-197: Precompiled contracts for optimal ate pairing check on the elliptic curve `alt_bn128`.
//!
//! ## Simple Summary
//!
//! Precompiled contracts for elliptic curve pairing operations are required in order to perform zkSNARK verification within the block gas limit.
//!
//! ## Abstract
//!
//! This EIP suggests to add precompiled contracts for a pairing function on a specific pairing-friendly elliptic curve. This can in turn be combined with EIP-196 to verify zkSNARKs in Ethereum smart contracts. The general benefit of zkSNARKs for Ethereum is that it will increase the privacy for users (because of the Zero-Knowledge property) and might also be a scalability solution (because of the succinctness and efficient verifiability property).
//!
//! ## Motivation
//!
//! Current smart contract executions on Ethereum are fully transparent, which makes them unsuitable for several use-cases that involve private information like the location, identity or history of past transactions. The technology of zkSNARKs could be a solution to this problem. While the Ethereum Virtual Machine can make use of zkSNARKs in theory, they are currently too expensive
//! to fit the block gas limit. Because of that, this EIP proposes to specify certain parameters for some elementary primitives that enable zkSNARKs so that they can be implemented more efficiently and the gas cost be reduced.
//!
//! Note that fixing these parameters will in no way limit the use-cases for zkSNARKs, it will even allow for incorporating some advances in zkSNARK research without the need for a further hard fork.
//!
//! Pairing functions can be used to perform a limited form of multiplicatively homomorphic operations, which are necessary for current zkSNARKs. This precompile can be used to run such computations within the block gas limit. This precompiled contract only specifies a certain check, and not an evaluation of a pairing function. The reason is that the codomain of a pairing function is a rather complex field which could provide encoding problems and all known uses of pairing function in zkSNARKs only require the specified check.
//!
//! ## Specification
//!
//! For blocks where `block.number >= BYZANTIUM_FORK_BLKNUM`, add a precompiled contracts for a bilinear function on groups on the elliptic curve `alt_bn128`. We will define the precompiled contract in terms of a discrete logarithm. The discrete logarithm is of course assumed to be hard to compute, but we will give an equivalent specification that makes use of elliptic curve pairing functions which can be efficiently computed below.
//!
//! Address: 0x8
//!
//! For a cyclic group `G` (written additively) of prime order `q` let `log_P: G -> F_q` be the discrete logarithm on this group with respect to a generator `P`, i.e. `log_P(x)` is the smallest non-negative integer `n` such that `n * P = x`.
//!
//! The precompiled contract is defined as follows, where the two groups `G_1` and `G_2` are defined by their generators `P_1` and `P_2` below. Both generators have the same prime order `q`.
//!
//! ```python
//! Input: (a1, b1, a2, b2, ..., ak, bk) from (G_1 x G_2)^k
//! Output: If the length of the input is incorrect or any of the inputs are not elements of
//!         the respective group or are not encoded correctly, the call fails.
//!         Otherwise, return one if
//!         log_P1(a1) * log_P2(b1) + ... + log_P1(ak) * log_P2(bk) = 0
//!         (in F_q) and zero else.
//! ```
//!
//! Note that `k` is determined from the length of the input. Following the section on the encoding below,
//! `k` is the length of the input divided by `192`. If the input length is not a multiple of `192`,
//! the call fails. Empty input is valid and results in returning one.
//!
//! In order to check that an input is an element of `G_1`, verifying the encoding of the coordinates and checking that they satisfy the curve equation (or is the encoding of infinity) is sufficient. For `G_2`, in addition to that, the order of the element has to be checked to be equal to the group order `q = 21888242871839275222246405745257275088548364400416034343698204186575808495617`.
//!
//! ### Definition of the groups
//!
//! The groups `G_1` and `G_2` are cyclic groups of prime order `q = 21888242871839275222246405745257275088548364400416034343698204186575808495617`.
//!
//! The group `G_1` is defined on the curve `Y^2 = X^3 + 3` over the field `F_p` with `p = 21888242871839275222246405745257275088696311157297823662689037894645226208583` with generator `P1 = (1, 2)`.
//!
//! The group `G_2` is defined on the curve `Y^2 = X^3 + 3/(i+9)` over a different field `F_p^2 = F_p[i] / (i^2 + 1)` (p is the same as above) with generator
//! ```python
//! P2 = (
//!   11559732032986387107991004021392285783925812861821192530917403151452391805634 * i +
//!   10857046999023057135944570762232829481370756359578518086990519993285655852781,
//!   4082367875863433681332203403145435568316851327593401208105741076214120093531 * i +
//!   8495653923123431417604973247489272438418190587263600148770280649306958101930
//! )
//! ```
//!
//! Note that `G_2` is the only group of order `q` of that elliptic curve over the field `F_p^2`. Any other generator of order `q` instead of `P2` would define the same `G_2`. However, the concrete value of `P2` is useful for skeptical readers who doubt the existence of a group of order `q`. They can be instructed to compare the concrete values of `q * P2` and `P2`.
//!
//!
//! ### Encoding
//!
//! Elements of `F_p` are encoded as 32 byte big-endian numbers. An encoding value of `p` or larger is invalid.
//!
//! Elements `a * i + b` of `F_p^2` are encoded as two elements of `F_p`, `(a, b)`.
//!
//! Elliptic curve points are encoded as a Jacobian pair `(X, Y)` where the point at infinity is encoded as `(0, 0)`.
//!
//! Note that the number `k` is derived from the input length.
//!
//! The length of the returned data is always exactly 32 bytes and encoded as a 32 byte big-endian number.
//!
//! ### Gas costs
//!
//! The gas costs of the precompiled contract are `80 000 * k + 100 000`, where `k` is the number of
//! points or, equivalently, the length of the input divided by 192.
//!
//! ## Rationale
//!
//! The specific curve `alt_bn128` was chosen because it is particularly well-suited for zkSNARKs, or, more specifically their verification building block of pairing functions. Furthermore, by choosing this curve, we can use synergy effects with `ZCash` and re-use some of their components and artifacts.
//!
//! The feature of adding curve and field parameters to the inputs was considered but ultimately rejected since it complicates the specification; the gas costs are much harder to determine and it would be possible to call the contracts on something which is not an actual elliptic curve or does not admit an efficient pairing implementation.
//!
//! A non-compact point encoding was chosen since it still allows to perform some operations in the smart contract itself (inclusion of the full y coordinate) and two encoded points can be compared for equality (no third projective coordinate).
//!
//! The encoding of field elements in `F_p^2` was chosen in this order to be in line with the big endian encoding of the elements themselves.
//!
//! ## Backwards Compatibility
//!
//! As with the introduction of any precompiled contract, contracts that already use the given addresses will change their semantics. Because of that, the addresses are taken from the "reserved range" below 256.
//!
//! ## Test Cases
//!
//! To be written.
//!
//! ## Implementation
//!
//! The precompiled contract can be implemented using elliptic curve pairing functions, more specifically, an optimal ate pairing on the `alt_bn128` curve, which can be implemented efficiently. In order to see that, first note that a pairing function `e: G_1 x G_2 -> G_T` fulfills the following properties (`G_1` and `G_2` are written additively, `G_T` is written multiplicatively):
//!
//! (1) `e(m * P1, n * P2) = e(P1, P2)^(m * n)`
//! (2) `e` is non-degenerate
//!
//! Now observe that
//! ```python
//! log_P1(a1) * log_P2(b1) + ... + log_P1(ak) * log_P2(bk) = 0 (in F_q)
//! ```
//! if and only if
//! ```python
//! e(P1, P2)^(log_P1(a1) * log_P2(b1) + ... + log_P1(ak) * log_P2(bk)) = 1 (in G_T)
//! ```
//!
//! Furthermore, the left hand side of this equation is equal to
//! ```python
//! e(log_P1(a1) * P1, log_P2(b1) * P2) * ... * e(log_P1(ak) * P1, log_P2(bk) * P2)
//! = e(a1, b1) * ... * e(ak, bk)
//! ```
//!
//! And thus, the precompiled contract can be implemented by verifying that
//! `e(a1, b1) * ... * e(ak, bk) = 1`
//!
//! Vitalik Buterin <vitalik@ethereum.org>, Christian Reitwiessner <chris@ethereum.org>, "EIP-197: Precompiled contracts for optimal ate pairing check on the elliptic curve `alt_bn128`," Ethereum Improvement Proposals, no. 197, February 2017. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-197>.

use crate::eip::Eip;

/// EIP-197: Precompiled contracts for optimal ate pairing check on the elliptic curve `alt_bn128`.
pub struct Eip197;

impl Eip for Eip197 {
    const NUMBER: u32 = 197;
}
