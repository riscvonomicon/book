# Enabling Extensions

This chapter explains how to use RISC-V target features. Rust support several
target features for the RISC-V, these can be listed with the following command.

```bash
rustc --print target-features --target=riscv32imac-unknown-none-elf
```

These features can be enabled through use of the `-C target-feature` flag. This
can be done using the `RUSTFLAGS="-Ctarget-feature=+<feature>"` or by adding to
the `.cargo/config.toml`.

For example, to build with the `zk` target feature, it is possible run:

```bash
RUSTFLAGS="-Ctarget-feature=+zk" cargo build
```

Or you can add the following to the `.cargo/config.toml` file of your project.

```toml
# NOTE: Replace `riscv32imac-unknown-none-elf` with your specific target
[target.riscv32imac-unknown-none-elf]
rustflags = ['-Ctarget-feature=+zk']
```

## The `crt-static` target feature

The `crt-static` feature statically links the C run-time libraries. This is
commonly needed for embedded targets.
