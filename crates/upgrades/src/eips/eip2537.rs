//! EIP-2537: Precompile for BLS12-381 curve operations.
//!
//! ## Abstract
//!
//! Add functionality to efficiently perform operations over the BLS12-381 curve, including those for BLS signature verification.
//!
//! Along with the curve arithmetic, multi-scalar-multiplication operations are included to efficiently aggregate public keys or individual signer's signatures during BLS signature verification.
//!
//! ## Motivation
//!
//! The motivation of this precompile is to add a cryptographic primitive that allows to get 120+ bits of security for operations over pairing friendly curve compared to the existing BN254 precompile that only provides 80 bits of security.
//!
//! ## Specification
//!
//! ### Constants
//!
//! | Name                | Value | Comment            |
//! |---------------------|-------|--------------------|
//! | `BLS12_G1ADD`         | 0x0b  | precompile address |
//! | `BLS12_G1MSM`         | 0x0c  | precompile address |
//! | `BLS12_G2ADD`         | 0x0d  | precompile address |
//! | `BLS12_G2MSM`         | 0x0e  | precompile address |
//! | `BLS12_PAIRING_CHECK` | 0x0f  | precompile address |
//! | `BLS12_MAP_FP_TO_G1`  | 0x10  | precompile address |
//! | `BLS12_MAP_FP2_TO_G2` | 0x11  | precompile address |
//!
//! We introduce *seven* separate precompiles to perform the following operations:
//!
//! - `BLS12_G1ADD` - to perform point addition in G1 (curve over base prime field) with a gas cost of `375` gas
//! - `BLS12_G1MSM` - to perform multi-scalar-multiplication (MSM) in G1 (curve over base prime field) with a gas cost formula defined in the corresponding section
//! - `BLS12_G2ADD` - to perform point addition in G2 (curve over quadratic extension of the base prime field) with a gas cost of `600` gas
//! - `BLS12_G2MSM` - to perform multi-scalar-multiplication (MSM) in G2 (curve over quadratic extension of the base prime field) with a gas cost formula defined in the corresponding section
//! - `BLS12_PAIRING_CHECK` - to perform a pairing operations between a set of *pairs* of (G1, G2) points a gas cost formula defined in the corresponding section
//! - `BLS12_MAP_FP_TO_G1` - maps base field element into the G1 point with a gas cost of `5500` gas
//! - `BLS12_MAP_FP2_TO_G2` - maps extension field element into the G2 point with a gas cost of `23800` gas
//!
//! A mapping functions specification is included as a separate [document](../assets/eip-2537/field_to_curve.md). This mapping function does NOT perform mapping of the byte string into a field element (as it can be implemented in many different ways and can be efficiently performed in EVM), but only does field arithmetic to map a field element into a curve point. Such functionality is required for signature schemes.
//!
//! ### Curve parameters
//!
//! The BLS12 curve is fully defined by the following set of parameters (coefficient `A=0` for all BLS12 curves):
//!
//! ```python
//! Base field modulus = p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
//! Fp - finite field of size p
//! Curve Fp equation: Y^2 = X^3+B (mod p)
//! B coefficient = 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004
//! Main subgroup order = q = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
//! Extension tower
//! Fp2 construction:
//! Fp quadratic non-residue = nr2 = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa
//! Fp2 is Fp[X]/(X^2-nr2)
//! Curve Fp2 equation: Y^2 = X^3 + B*(v+1) where v is the square root of nr2
//! Fp6/Fp12 construction:
//! Fp2 cubic non-residue c0 = 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
//! Fp2 cubic non-residue c1 = 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
//! Twist parameters:
//! Twist type: M
//! B coefficient for twist c0 = 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004
//! B coefficient for twist c1 = 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004
//! Generators:
//! H1:
//! X = 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb
//! Y = 0x08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1
//! H2:
//! X c0 = 0x024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8
//! X c1 = 0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e
//! Y c0 = 0x0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801
//! Y c1 = 0x0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be
//! Pairing parameters:
//! |x| (miller loop scalar) = 0xd201000000010000
//! x is negative = true
//! ```
//!
//! One should note that base field modulus `p` is equal to `3 mod 4` that allows an efficient square root extraction, although as described below gas cost of decompression is larger than gas cost of supplying decompressed point data in `calldata`.
//!
//! ### Fields and Groups
//!
//! Field Fp is defined as the finite field of size `p` with elements represented as integers between 0 and p-1 (both inclusive).
//!
//! Field Fp2 is defined as `Fp[X]/(X^2-nr2)` with elements  `el = c0 + c1 * v`, where `v` is the formal square root of `nr2` represented as integer pairs `(c0,c1)`.
//!
//! Group G1 is defined as a set of Fp pairs (points) `(x,y)` such that either `(x,y)` is  `(0,0)` or `x,y` satisfy the curve Fp equation.
//!
//! Group G2 is defined as a set of Fp2 pairs (points) `(x',y')` such that either `(x,y)` is `(0,0)` or `(x',y')` satisfy the curve Fp2 equation.
//!
//! ### Fine points and encoding of base elements
//!
//! #### Field elements encoding:
//!
//! In order to produce inputs to an operation, one encodes elements of the base field and the extension field.
//!
//! A base field element (Fp) is encoded as `64` bytes by performing the `BigEndian` encoding of the corresponding (unsigned) integer. Due to the size of `p`, the top `16` bytes are always zeroes. `64` bytes are chosen to have `32` byte aligned ABI (representable as e.g. `bytes32[2]` or `uint256[2]` with the latter assuming the `BigEndian` encoding). The corresponding integer **must** be less than field modulus.
//!
//! For elements of the quadratic extension field (Fp2), encoding is byte concatenation of individual encoding of the coefficients totaling in `128` bytes for a total encoding. For an Fp2 element in a form `el = c0 + c1 * v` where `v` is the formal square root of a quadratic non-residue and `c0` and `c1` are Fp elements the corresponding byte encoding will be `encode(c0) || encode(c1)` where `||` means byte concatenation (or one can use `bytes32[4]` or `uint256[4]` in terms of Solidity types).
//!
//! *Note on the top `16` bytes being zero*: it is required that an encoded element is "in a field", which means strictly `< modulus`. In `BigEndian` encoding it automatically means that for a modulus that is just `381` bit long the top `16` bytes in `64` bytes encoding are zeroes and this **must** be checked even if only a subslice of input data is used for actual decoding.
//!
//! On inputs that can not be a valid encodings of field elements the precompile *must* return an error.
//!
//! #### Encoding of points in G1/G2:
//!
//! Points of G1 and G2 are encoded as byte concatenation of the respective encodings of the `x` and `y` coordinates. Total encoding length for a G1 point is thus `128` bytes and for a G2 point is `256` bytes.
//!
//! #### Point of infinity encoding:
//!
//! Also referred to as the "zero point". For BLS12 curves, the point with coordinates `(0, 0)` (zeroes in Fp or Fp2) is *not* on the curve, so a sequence of `128` resp. `256` zero bytes, which naively would decode as `(0, 0)` is instead used by convention to encode the point of infinity of G1 resp. G2.
//!
//! #### Encoding of scalars for multiplication operation:
//!
//! A scalar for the multiplication operation is encoded as `32` bytes by performing `BigEndian` encoding of the corresponding (unsigned) integer. The corresponding integer is **not** required to be less than or equal to main subgroup order `q`.
//!
//! #### Behavior on empty inputs:
//!
//! Certain operations have variable length input, such as MSMs (takes a list of pairs `(point, scalar)`), or pairing (takes a list of `(G1, G2)` points). While their behavior is well-defined (from an arithmetic perspective) on empty inputs, this EIP discourages such use cases and variable input length operations must return an error if the input is empty.
//!
//! ### ABI for operations
//!
//! #### ABI for G1 addition
//!
//! G1 addition call expects `256` bytes as an input that is interpreted as byte concatenation of two G1 points (`128` bytes each). Output is an encoding of addition operation result - single G1 point (`128` bytes).
//!
//! Error cases:
//!
//! - Invalid coordinate encoding
//! - An input is neither a point on the G1 elliptic curve nor the infinity point
//! - Input has invalid length
//!
//! Note:
//!
//! There is no subgroup check for the G1 addition precompile.
//!
//! #### ABI for G1 MSM
//!
//! G1 MSM call expects `160*k` (`k` being a **positive** integer) bytes as an input that is interpreted as byte concatenation of `k` slices each of them being a byte concatenation of encoding of a G1 point (`128` bytes) and encoding of a scalar value (`32` bytes). Output is an encoding of MSM operation result - a single G1 point (`128` bytes).
//!
//! Error cases:
//!
//! - Invalid coordinate encoding
//! - An input is neither a point on the G1 elliptic curve nor the infinity point
//! - An input is on the G1 elliptic curve but not in the correct subgroup
//! - Input has invalid length
//!
//! #### ABI for G2 addition
//!
//! G2 addition call expects `512` bytes as an input that is interpreted as byte concatenation of two G2 points (`256` bytes each). Output is an encoding of addition operation result - a single G2 point (`256` bytes).
//!
//! Error cases:
//!
//! - Invalid coordinate encoding
//! - An input is neither a point on the G2 elliptic curve nor the infinity point
//! - Input has invalid length
//!
//! Note:
//!
//! There is no subgroup check for the G2 addition precompile.
//!
//! #### ABI for G2 MSM
//!
//! G2 MSM call expects `288*k` (`k` being a **positive** integer) bytes as an input that is interpreted as byte concatenation of `k` slices each of them being a byte concatenation of encoding of G2 point (`256` bytes) and encoding of a scalar value (`32` bytes). Output is an encoding of MSM operation result - a single G2 point (`256` bytes).
//!
//! Error cases:
//!
//! - Invalid coordinate encoding
//! - An input is neither a point on the G2 elliptic curve nor the infinity point
//! - An input is on the G2 elliptic curve but not in the correct subgroup
//! - Input has invalid length
//!
//! #### ABI for pairing check
//!
//! Pairing check call expects `384*k`  (`k` being a **positive** integer)  bytes as an inputs that is interpreted as byte concatenation of `k` slices. Each slice has the following structure:
//!
//! - `128` bytes of G1 point encoding
//! - `256` bytes of G2 point encoding
//!
//! Each point is expected to be in the subgroup of order `q`.
//!
//! It checks the equation `e(P1, Q1) * e(P2, Q2) * ... * e(Pk, Qk) == 1` in the pairing target field where `e` is the pairing operation. Output is `32` bytes where first `31` bytes are equal to `0x00` and the last byte is either `0x00` (false) or `0x01` (true).
//!
//! Error cases:
//!
//! - Invalid coordinate encoding
//! - An input is neither a point on its respective elliptic curve nor the infinity point
//! - An input is on its respective elliptic curve but not in the correct subgroup
//! - Input has invalid length
//!
//! #### ABI for mapping Fp element to G1 point
//!
//! Field-to-curve call expects `64` bytes as an input that is interpreted as an element of Fp. Output of this call is `128` bytes and is an encoded G1 point.
//!
//! Error cases:
//!
//! - Input has invalid length
//! - Input is not correctly encoded
//!
//! #### ABI for mapping Fp2 element to G2 point
//!
//! Field-to-curve call expects `128` bytes as an input that is interpreted as an element of Fp2. Output of this call is `256` bytes and is an encoded G2 point.
//!
//! Error cases:
//!
//! - Input has invalid length
//! - Input is not correctly encoded
//!
//! ### Gas burning on error
//!
//! Following the current state of all other precompiles, if a call to one of the precompiles in this EIP results in an error then all the gas supplied along with a `CALL` or `STATICCALL` is burned.
//!
//! ### `DDoS` protection
//!
//! A sane implementation of this EIP *should not* contain potential infinite loops (it is possible and not even hard to implement all the functionality without `while` loops) and the gas schedule accurately reflects the time spent on computations of the corresponding function (precompiles pricing reflects an amount of gas consumed in the worst case where such a case exists).
//!
//! ### Gas schedule
//!
//! Assuming `EcRecover` precompile as a baseline.
//!
//! #### G1 addition
//!
//! `375` gas
//!
//! #### G1 multiplication
//!
//! `12000` gas
//!
//! #### G2 addition
//!
//! `600` gas
//!
//! #### G2 multiplication
//!
//! `22500` gas
//!
//! #### G1/G2 MSM
//!
//! MSMs are expected to be performed by Pippenger's algorithm (we can also say that it **must** be performed by Pippenger's algorithm to have a speedup that results in a discount over naive implementation by multiplying each pair separately and adding the results). For this case there was a table prepared for discount in case of `k <= 128` points in the MSM with a discount cap `max_discount` for `k > 128`.
//!
//! The call cost is calculated as `(k * multiplication_cost * discount) // multiplier` where `multiplier = 1000`, `k` is a number of (scalar, point) pairs for the call, `multiplication_cost` is a corresponding G1/G2 multiplication cost presented above and `//` is an integer division.
//!
//! G1 and G2 are priced separately, each having their own discount table and `max_discount`.
//!
//! ##### G1 discounts
//!
//! Discounts table for G1 MSM as a vector of pairs `[k, discount]`:
//!
//! ```python
//! [[1, 1000], [2, 949], [3, 848], [4, 797], [5, 764], [6, 750], [7, 738], [8, 728], [9, 719], [10, 712], [11, 705], [12, 698], [13, 692], [14, 687], [15, 682], [16, 677], [17, 673], [18, 669], [19, 665], [20, 661], [21, 658], [22, 654], [23, 651], [24, 648], [25, 645], [26, 642], [27, 640], [28, 637], [29, 635], [30, 632], [31, 630], [32, 627], [33, 625], [34, 623], [35, 621], [36, 619], [37, 617], [38, 615], [39, 613], [40, 611], [41, 609], [42, 608], [43, 606], [44, 604], [45, 603], [46, 601], [47, 599], [48, 598], [49, 596], [50, 595], [51, 593], [52, 592], [53, 591], [54, 589], [55, 588], [56, 586], [57, 585], [58, 584], [59, 582], [60, 581], [61, 580], [62, 579], [63, 577], [64, 576], [65, 575], [66, 574], [67, 573], [68, 572], [69, 570], [70, 569], [71, 568], [72, 567], [73, 566], [74, 565], [75, 564], [76, 563], [77, 562], [78, 561], [79, 560], [80, 559], [81, 558], [82, 557], [83, 556], [84, 555], [85, 554], [86, 553], [87, 552], [88, 551], [89, 550], [90, 549], [91, 548], [92, 547], [93, 547], [94, 546], [95, 545], [96, 544], [97, 543], [98, 542], [99, 541], [100, 540], [101, 540], [102, 539], [103, 538], [104, 537], [105, 536], [106, 536], [107, 535], [108, 534], [109, 533], [110, 532], [111, 532], [112, 531], [113, 530], [114, 529], [115, 528], [116, 528], [117, 527], [118, 526], [119, 525], [120, 525], [121, 524], [122, 523], [123, 522], [124, 522], [125, 521], [126, 520], [127, 520], [128, 519]]
//! ```
//!
//! `max_discount = 519`
//!
//! ##### G2 discounts
//!
//! Discounts table for G2 MSM as a vector of pairs `[k, discount]`:
//!
//! ```python
//! [[1, 1000], [2, 1000], [3, 923], [4, 884], [5, 855], [6, 832], [7, 812], [8, 796], [9, 782], [10, 770], [11, 759], [12, 749], [13, 740], [14, 732], [15, 724], [16, 717], [17, 711], [18, 704], [19, 699], [20, 693], [21, 688], [22, 683], [23, 679], [24, 674], [25, 670], [26, 666], [27, 663], [28, 659], [29, 655], [30, 652], [31, 649], [32, 646], [33, 643], [34, 640], [35, 637], [36, 634], [37, 632], [38, 629], [39, 627], [40, 624], [41, 622], [42, 620], [43, 618], [44, 615], [45, 613], [46, 611], [47, 609], [48, 607], [49, 606], [50, 604], [51, 602], [52, 600], [53, 598], [54, 597], [55, 595], [56, 593], [57, 592], [58, 590], [59, 589], [60, 587], [61, 586], [62, 584], [63, 583], [64, 582], [65, 580], [66, 579], [67, 578], [68, 576], [69, 575], [70, 574], [71, 573], [72, 571], [73, 570], [74, 569], [75, 568], [76, 567], [77, 566], [78, 565], [79, 563], [80, 562], [81, 561], [82, 560], [83, 559], [84, 558], [85, 557], [86, 556], [87, 555], [88, 554], [89, 553], [90, 552], [91, 552], [92, 551], [93, 550], [94, 549], [95, 548], [96, 547], [97, 546], [98, 545], [99, 545], [100, 544], [101, 543], [102, 542], [103, 541], [104, 541], [105, 540], [106, 539], [107, 538], [108, 537], [109, 537], [110, 536], [111, 535], [112, 535], [113, 534], [114, 533], [115, 532], [116, 532], [117, 531], [118, 530], [119, 530], [120, 529], [121, 528], [122, 528], [123, 527], [124, 526], [125, 526], [126, 525], [127, 524], [128, 524]]
//! ```
//!
//! `max_discount = 524`
//!
//! #### Pairing check operation
//!
//! The cost of the pairing check operation is `32600*k + 37700` where `k` is a number of pairs.
//!
//! #### Fp-to-G1 mapping operation
//!
//! Fp -> G1 mapping is `5500` gas.
//!
//! #### Fp2-to-G2 mapping operation
//!
//! Fp2 -> G2 mapping is `23800` gas
//!
//! #### Gas schedule clarifications for the variable-length input
//!
//! For MSM and pairing functions, the gas cost depends on the input length. The current state of how the gas schedule is implemented in major clients (at the time of writing) is that the gas cost function does *not* perform any validation of the length of the input and never returns an error. So we present a list of rules how the gas cost functions **must** be implemented to ensure consistency between clients and safety.
//!
//! ##### Gas schedule clarifications for G1/G2 MSM
//!
//! Define a constant `LEN_PER_PAIR` that is equal to `160` for G1 operation and to `288` for G2 operation. Define a function `discount(k)` following the rules in the corresponding section, where `k` is number of pairs.
//!
//! The following pseudofunction reflects how gas should be calculated:
//!
//! ```python
//! k = floor(len(input) / LEN_PER_PAIR);
//! if k == 0 {
//!   return 0;
//! }
//! gas_cost = k * multiplication_cost * discount(k) // multiplier;
//! return gas_cost;
//! ```
//!
//! We use floor division to get the number of pairs. If the length of the input is not divisible by `LEN_PER_PAIR` we still produce *some* result, but later on the precompile will return an error. Also, the case when `k = 0` is safe: `CALL` or `STATICCALL` cost is non-zero, and the case with formal zero gas cost is already used in `Blake2f` precompile. In any case, the main precompile routine **must** produce an error on such an input because it violated encoding rules.
//!
//! ##### Gas schedule clarifications for pairing
//!
//! Define a constant `LEN_PER_PAIR = 384`;
//!
//! The following pseudofunction reflects how gas should be calculated:
//!
//! ```python
//! k = floor(len(input) / LEN_PER_PAIR);
//! gas_cost = 32600*k + 37700;
//! return gas_cost;
//! ```
//!
//! We use floor division to get the number of pairs. If the length of the input is not divisible by `LEN_PER_PAIR` we still produce *some* result, but later on the precompile will return an error (the precompile routine **must** produce an error on such an input because it violated encoding rules).
//!
//! ## Rationale
//!
//! The motivation section covers a total motivation to have operations over the BLS12-381 curves available. We also extend a rationale for more specific fine points.
//!
//! ### MSM as a separate call
//!
//! Explicit separate MSM operation that allows one to save execution time (so gas) by both the algorithm used (namely Pippenger's algorithm) and (usually forgotten) by the fact that `CALL` operation in Ethereum is expensive (at the time of writing), so one would have to pay non-negligible overhead if e.g. for MSM of `100` points would have to call the multiplication precompile `100` times and addition for `99` times (roughly `138600` would be saved).
//!
//! ### No dedicated MUL call
//!
//! Dedicated MUL precompiles which perform single G1/G2 point by scalar multiplication have exactly the same ABI as MSM with `k == 1`.
//! MSM has to inspect the input length to reject inputs of invalid lengths. Therefore, it should recognize the case of `k == 1` and invoke the underlying implementation of single point multiplication to avoid the overhead of more complex multi-scalar multiplication algorithm.
//!
//! ## Backwards Compatibility
//!
//! There are no backward compatibility questions.
//!
//! ### Subgroup checks
//!
//! MSMs and pairings MUST perform a subgroup check.
//! Implementations SHOULD use the optimized subgroup check method detailed in a dedicated [document](../assets/eip-2537/fast_subgroup_checks.md).
//! On any input that fails the subgroup check, the precompile MUST return an error.
//! As endomorphism acceleration requires input on the correct subgroup, implementers MAY use endomorphism acceleration.
//!
//! ### Field to curve mapping
//!
//! The algorithms and set of parameters for SWU mapping method are provided by a separate [document](../assets/eip-2537/field_to_curve.md)
//!
//! ## Test Cases
//!
//! Due to the large test parameters space, we first provide properties that various operations must satisfy. We use additive notation for point operations, capital letters (`P`, `Q`) for points, small letters (`a`, `b`) for scalars. The generator for G1 is labeled as `G`, the generator for G2 is labeled as `H`, otherwise we assume random points on a curve in a correct subgroup. `0` means either scalar zero or point at infinity. `1` means either scalar one or multiplicative identity. `group_order` is the main subgroup order. `e(P, Q)` means pairing operation where `P` is in G1, `Q` is in G2.
//!
//! Required properties for basic ops (add/multiply):
//!
//! - Commutativity: `P + Q = Q + P`
//! - Identity element: `P + 0 = P`
//! - Additive negation: `P + (-P) = 0`
//! - Doubling `P + P = 2*P`
//! - Subgroup check: `group_order * P = 0`
//! - Trivial multiplication check: `1 * P = P`
//! - Multiplication by zero: `0 * P = 0`
//! - Multiplication by the unnormalized scalar `(scalar + group_order) * P = scalar * P`
//!
//! Required properties for pairing operation:
//!
//! - Bilinearity `e(a*P, b*Q) = e(a*b*P, Q) = e(P, a*b*Q)`
//! - Non-degeneracy `e(P, Q) != 1`
//! - `e(P, 0*Q) = e(0*P, Q) = 1`
//! - `e(P, -Q) = e(-P, Q)`
//!
//! Alex Vlasov (@shamatar), Kelly Olson (@ineffectualproperty), Alex Stokes (@ralexstokes), Antonio Sanso (@asanso), "EIP-2537: Precompile for BLS12-381 curve operations," Ethereum Improvement Proposals, no. 2537, February 2020. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-2537>.

use crate::eip::Eip;

/// EIP-2537: Precompile for BLS12-381 curve operations.
pub struct Eip2537;

impl Eip for Eip2537 {
    const NUMBER: u32 = 2537;
}
