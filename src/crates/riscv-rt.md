# `riscv-rt` crate

[Repository](https://github.com/rust-embedded/riscv-rt) | [Documentation](https://docs.rs/riscv-rt/latest/riscv_rt/)

The `riscv-rt` crate provides a "Minimal runtime / startup for RISC-V CPU's". It makes sure that:

* Set up`.data`, `.bss` sections correctly
* Set up traps / interrupts in the correct place
* Allocate a stack per *hardware thread* (hart)

```rust
#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
	loop {}
}
```


# Reset to `main` on `riscv-rt`

This section describes all the code that gets executed from a CPU reset to the `main` function.

The `link.x` file defines the regions of the resulting [ELF] file. What is important that the the order of items in the `.text` section of the binary

```asm
.section .init, "ax"
.global _start
_start:
```

The `.section .init` ensures that the following code is put in the `.init` section, which the `link.x` puts at the start of the `.text` region. Then, we define a `_start` symbol and label this section to begin here. The `_start` section is generally assumed to be the entry point for [ELF] binaries.

```asm
// Only for rv32
lui ra, %hi(_abs_start) // ra <- addr_of(_abs_start) & 0xFFFF_F000
jr %lo(_abs_start)(ra)
	
// Only for rv64
.option push
.option norelax // to prevent an unsupported R_RISCV_ALIGN relocation from being generated
1:
	auipc ra, %pcrel_hi(1f)
	ld ra, %pcrel_lo(1b)(ra)
	jr ra
.align  3
1:
.dword _abs_start
.option pop
_abs_start:
```

This code seems quite crazy and uses some strange assembly syntax. Let us first dive into the 32-bit version as it is the easier one.

It consists of two instructions. The first instruction loads the upper 20 bits of the `_abs_start` symbol, which is defined at the end, into the `ra` (return address) register. The second instruction jumps to the `ra` register offset by the lower 12 bits of 

```asm
_abs_start:
    .option norelax
    .cfi_startproc
    .cfi_undefined ra
    #[cfg(feature = "s-mode")]
    {
	    csrw sie, 0
	    csrw sip, 0
	}
	
    #[cfg(not(feature = "s-mode"))]
    {
	    csrw mie, 0
	    csrw mip, 0
	}
    
    li  x1, 0
    li  x2, 0
    li  x3, 0
    li  x4, 0
    li  x5, 0
    li  x6, 0
    li  x7, 0
    li  x8, 0
    li  x9, 0
    // a0..a2 (x10..x12) skipped
    li  x13, 0
    li  x14, 0
    li  x15, 0
    li  x16, 0
    li  x17, 0
    li  x18, 0
    li  x19, 0
    li  x20, 0
    li  x21, 0
    li  x22, 0
    li  x23, 0
    li  x24, 0
    li  x25, 0
    li  x26, 0
    li  x27, 0
    li  x28, 0
    li  x29, 0
    li  x30, 0
    li  x31, 0

    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop",
    #[cfg(all(not(feature = "single-hart"), feature = "s-mode"))]
    "mv t2, a0 // the hartid is passed as parameter by SMODE",
    #[cfg(all(not(feature = "single-hart"), not(feature = "s-mode")))]
    "csrr t2, mhartid",
    #[cfg(not(feature = "single-hart"))]
    "lui t0, %hi(_max_hart_id)
    add t0, t0, %lo(_max_hart_id)
    bgtu t2, t0, abort",
    "// Allocate stacks
    la sp, _stack_start
    lui t0, %hi(_hart_stack_size)
    add t0, t0, %lo(_hart_stack_size)",
    #[cfg(all(not(feature = "single-hart"), riscvm))]
    "mul t0, t2, t0",
    #[cfg(all(not(feature = "single-hart"), not(riscvm)))]
    "beqz t2, 2f  // Jump if single-hart
    mv t1, t2
    mv t3, t0
1:
    add t0, t0, t3
    addi t1, t1, -1
    bnez t1, 1b
2:  ",
    "sub sp, sp, t0

    // Set frame pointer
    add s0, sp, zero

    jal zero, _start_rust

    .cfi_endproc",
```

[^1]: https://twilco.github.io/riscv-from-scratch/2019/04/27/riscv-from-scratch-2.html