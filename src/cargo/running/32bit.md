# RISC-V 32-bit

To run 32-bit RISC-V code, it is possible to use the [QEMU] emulator. How to
test and run this code, depends on the contents of this code. For libraries that
contain arithmetic instructions, it is possible to use
[`ralte32`](https://github.com/riscvonomicon/ralte32) or the
`riscv32gc-unknown-linux-gnu` nightly target.

[QEMU]: https://www.qemu.org/
