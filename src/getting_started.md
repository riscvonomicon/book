# Getting Started

To get started with using RISC-V and Rust together, we will need a couple of
things.

1. Cross-compilation linker
2. ISA emulation environment
3. Rust toolchain for RISC-V

## Cross-compilation linker

It is generally advised to install the GNU RISC-V Toolchain for the
corresponding target. This differs between from target to target. Generally,
it is advised to install both the RV32 and RV64 versions. These can found
[here][riscv-gnu-toolchain].

Below is a table for different platforms and which commands can be executed to
get the RISC-V GNU Toolchain.

| Platform | Command | Link |
| -------- | ------- | ---- |
| ArchLinux | `paru -S riscv-gnu-toolchain-bin` | [Link](https://aur.archlinux.org/packages/riscv-gnu-toolchain-bin) |
| Ubuntu Linux | Install from source | |
| macOS | `brew tap riscv-software-src/riscv && brew install riscv-tools` | [Link](https://github.com/riscv-software-src/homebrew-riscv) |

## ISA emulation environment

- QEMU
- Spike

## Rust toolchain for RISC-V

Rust supports several variants of the RISC-V instruction set. All the targets
can be listed with the following command.

```bash
rustup target list | grep '^riscv'
```

The target determines the register sizes and which instructions to use. For
example, the `riscv32imac-unknown-none-elf` target includes the `m`
(Multiplication), `a` (Atomics) and `c` (Compressed) instructions, where the
`riscv32imac-unknown-none-elf` target only includes the base instructions.

To install the toolchain for a specific target, you can run the following
command.

```bash
# Replace 'riscv64gc-unknown-linux-gnu' with the desired target
rustup target add riscv64gc-unknown-linux-gnu
```

[riscv-gnu-toolchain]: https://github.com/riscv-collab/riscv-gnu-toolchain
