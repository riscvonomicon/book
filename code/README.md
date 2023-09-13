# riscvonomicon code examples

This directory contains many of larger code examples that are used in the
[riscvonomicon].

## Running tests

To run the 64-bit tests, you can run:

```bash
cargo +nightly test --target riscv64gc-unknown-linux-gnu
```

To run the 32-bit tests, you can run:

```bash
cargo +nightly run --example rv32-tests --target riscv32imac-unknown-none-elf
```

[riscvonomicon]: https://riscvonomicon.github.io/book