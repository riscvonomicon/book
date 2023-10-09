# Bare Metal

Getting started with Bare Metal.

```sh
# 32-bit target
# Alternatives: riscv32imac, riscv32imc
rustup target add riscvi-unknown-none-elf

# 64-bit target
# Alternatives: riscv64imac
rustup target add riscv64gc-unknown-none-elf
```


A very minimal example of a binary can ran as follows:

```rust
#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
extern "C" fn _start() -> ! {
	// NOTE:
	// The `.data` and `.bss` sections are not initiialized.
	// You might use the `r0` crate for this. Take a look at the section
	// on this crate.

	loop {}
}
```

A safer way that handles all initialization for you is to use the `riscv-rt` crate.

```sh
cargo add riscv-rt panic-halt
```

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