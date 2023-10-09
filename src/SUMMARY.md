# Summary

[Introduction](./introduction.md)
[Getting Started](./getting_started.md)

# Basics

- [Bare Metal](./basics/bare-metal.md)
- [Linux Binary]()

# Tooling

- [Cargo and RISC-V](./cargo/mod.md)
  - [Build](./cargo/building.md)
  - [Run & Test](./cargo/running/mod.md)
    - [RISC-V 32-bit](./cargo/running/32bit.md)
    - [RISC-V 64-bit](./cargo/running/64bit.md)
  - [Fuzz](./cargo/fuzz.md)
  - [Formally Verify](./cargo/verify.md)
- [Important Crates]()
  - [`riscv` crate]()
  - [`riscv-rt` crate](./crates/riscv-rt.md)
  - [`r0` crate]()

## Important Crates

- [`riscv` crate]()

# Atomics (A) Extension

# Bit Manipulation (B) Extension

# Compressed Instructions (C) Extension

# Vector (V) Extension

- [Overview](./extensions/v/README.md)
- [Getting Started](./extensions/v/getting-started.md)
- [Memory Operations](./extensions/v/memory-operations.md)
- [Integer Arithmetic](./extensions/v/integer-arithmetic.md)
- [Floating-Point Arithmetic](./extensions/v/floating-point-arithmetic.md)
- [Fixed-Point Arithmetic](./extensions/v/fixed-point-arithmetic.md)
- [Masks and Conditionals](./extensions/v/masks-and-conditionals.md)
- [Permutations](./extensions/v/permutations.md)
- [Tail Policy](./extensions/v/tail-policy.md)
- [Mask Policy](./extensions/v/mask-policy.md)
- [Cookbook](./extensions/v/cookbook/mod.md)
  - [Array Element-Wise Addition](./extensions/v/cookbook/vector-add.md)


# Scalar Cryptography (Zk) Extension

- [Overview](./extensions/zk/mod.md)
- [Bit Manipulation Instructions](./extensions/zk/b/overview.md)
- [AES Block Cipher \(Zkned\)](./extensions/zk/zkned/overview.md)
	- [Usage: 32-bit](./extensions/zk/zkned/32bit.md)
	- [Usage: 64-bit](./extensions/zk/zkned/64bit.md)
- [SHA-2 Hash Function \(Zknh\)](./extensions/zk/zknh/overview.md)
- [SM4 Block Cipher \(Zksed\)](./extensions/zk/zksed/overview.md)
- [SM3 Hash Function \(Zksh\)](./extensions/zk/zksed/overview.md)
- [Entropy Source \(Zkr\)](./extensions/zk/zkr/overview.md)
- [Data Independent Execution Latency \(Zkt\)](./extensions/zk/zkt/overview.md)


<!--
- [RISC-V Extensions](./extensions/mod.md)
  - [Enabling Extensions](./extensions/enabling.md)
  - [Compressed ("C")]()
  - [Multiplication ("M")]()
  - [Atomics ("A")]()
  - [Bit Manipulation ("B")]()
  - [Control Status Registers ("Zcsr")]()
  - [Scalar Cryptography ("Zk"))](./extensions/zk/mod.md)

-->
