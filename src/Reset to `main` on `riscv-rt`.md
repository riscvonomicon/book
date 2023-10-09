The `riscv-rt` serves as a base for embedded environments to set up a simple environment to run some embedded [RISC-V] code. This post describes all the steps that the `riscv-rt` crate takes to boot into its minimal runtime. The article article as a resource for how use `riscv-rt` and how to utilize or fork it to meet your specific needs. This post assumes some familiarity with [RISC-V assembly], embedded development, linkers and [Rust] but many deeper concepts are explained.

The article is divided into several sections. First, the post discusses the linker file. Afterward, a section describes all the code from when a reset happens to when the main function is called. Lastly, a section goes over the trap handling infrastructure. 

## The Linkerfile `link.x`

The `link.x` file defines the regions of the resulting [ELF] file. What is important that the the order of items in the `.text` section of the binary


## Reset to `main`

```asm
.section .init, "ax"
.global _start
_start:
```

The `.section .init` ensures that the following code is put in the `.init` section, which the `link.x` puts at the start of the `.text` region. The `"ax"` defines the flags off this section. The `a` signifying "allocate" (i.e. actually load the data for this section when running).  The `x` signifying "execute" (i.e. the data in this section is executable). Then, we place this location as `_start` in the global symbol table. This allows linkers to perform *Linker Magic*. The `_start` section is generally assumed to be the entry point for [ELF] binaries.

```asm
// Only for rv32
lui ra, %hi(_abs_start)      // ra <- addr_of(_abs_start) & 0xFFFF_F000
jr %lo(_abs_start)(ra)       // pc <- ra + addr_of(_abs_start) & 0x0000_0FFF
	
// Only for rv64
.option push
.option norelax // to prevent an unsupported R_RISCV_ALIGN relocation from being generated
1:
	auipc ra, %pcrel_hi(1f)  // ra <- pc + (addr_of(1) - pc) & 0xFFFF_FFF0
	ld ra, %pcrel_lo(1b)(ra)
	jr ra
.align  3
1:
.dword _abs_start
.option pop
_abs_start:
```

This code seems quite crazy and uses some strange RISC-V assembly syntax. Essentially, the entire goal is to jump to the label `_abs_start` at the end, while dealing with any weird *Linker Magic*.

The 32-bit version consists of two instructions. The first instruction loads the upper 20 bits of the `_abs_start` symbol, which is defined at the end, into the `ra` (return address) register. The second instruction jumps to the `ra` register offset by the lower 12 bits of the `_abs_start`. <!-- Fact check -->.  This is essentially a very convoluted way to ensure a jump to `_abs_start` always succeeds no matter the alignment.

The 64-bit version uses quite a lot more trickery.  We start from the back with the `.dword _abs_start`. This stores the address of the `_abs_start` label, which is defined afterwards.

```asm
.option norelax
.cfi_startproc
.cfi_undefined ra
```

```asm
csrw mie, 0 // Disable all interrupts
csrw mip, 0 // Clear all pending interrupts
```

```asm
li  x1, 0
li  x2, 0
li  x3, 0
li  x4, 0
li  x5, 0
li  x6, 0
li  x7, 0
li  x8, 0
li  x9, 0
// a0..a2 (x10..x12) skipped, since these will be set by later.
li  x13, 0
li  x14, 0
li  x15, 0

// This code is not included for the "E" extension, since this changes the
// register count to 16 instead of 32.
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
```

```asm
.option push
.option norelax
la gp, __global_pointer$
.option pop
```

```asm
// t2 <- hardware thread identifier (hartid)
csrr t2, mhartid

// jump to abort if hartid > _max_hart_id
lui t0, %hi(_max_hart_id)
add t0, t0, %lo(_max_hart_id)
bgtu t2, t0, abort
```

```asm
// Allocate stack
la sp, _stack_start

// t0 <- size of stack per hardware thread (hart)
lui t0, %hi(_hart_stack_size)
add t0, t0, %lo(_hart_stack_size)

// NOTE: This gets replaced by a multiplication coroutine on
// platforms without the "M" extension.
mul t0, t2, t0
sub sp, sp, t0
```

```asm
// Set frame pointer
add s0, sp, zero
```

```asm
// Jump to _start_rust
jal zero, _start_rust

.cfi_endproc
```

## Trap Handling Infrastructure

[^1]: https://twilco.github.io/riscv-from-scratch/2019/04/27/riscv-from-scratch-2.html
