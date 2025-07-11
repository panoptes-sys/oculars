# eva-asm

[![Crates.io](https://img.shields.io/crates/v/eva-asm.svg)](https://crates.io/crates/eva-asm)
[![Docs.rs](https://docs.rs/eva-asm/badge.svg)](https://docs.rs/eva-asm)
[![License: MIT](https://img.shields.io/crates/l/eva-asm)](#license)

**Expressive, correct and beautiful representations of EVM instructions, opcodes and mnemonics.**

## Examples

- Mnemonics

```rust
use eva_asm::opcode::Mnemonic;

let gas = Mnemonic::GAS;
assert_eq!(gas, 0x5A);
```

- Operation codes

```rust
use eva_asm::opcode::{Mnemonic, Gas};
let gas = OpCode::Known(Mnemonic::GAS);
assert_eq!(gas, 0x5A);

let unknown = OpCode::Unknown(0xF);
assert!(unknown.is_unknown());
```

- Instructions
```rust
use eva_asm::instructions::Push;

let push: Push<4> = Push::new([0xA]; 4);
assert!(push.is_push());
```
