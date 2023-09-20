# 32-bit AES

This section explains how to use the `aes32esmi`, `aes32esi`, `aes32dsi` and
`aes32dsmi` instructions in the `Zkne` extension to simplify and speed up the
implementation of the [Advanced Encryption Standard][AES]. The instructions can
be used to implement AES128, AES196 and AES256. A talk at the RISC-V summit[^1]
claims a speed-up of ~4x and a code size reduction of 0.3x[^1].

> ⚠️ **WARNING** ⚠️
>
> It is especially difficult to implement cryptography correctly and securely.
> If you can use a existing implementation that has been battle tested, you
> probably should. Still, this page exists to show how you would go about using
> this extension.

## Encryption

The `aes32esmi` instruction helps with implementing the middle rounds of AES.
It performs a byte substitution, mixing of columns and adding the roundkey. The
`aes32esi` instruction is used for the last round of the AES and performs a
byte substitution and adding the roundkey. An rust equivalent implementation of
the instructions would look like:

```rust,no_run,no_playground
static SBOX: [u8; 256] = [
    // ...
];

fn xt2(x: u8) -> u8 {
    (x << 1) ^ if x & 0x80 != 0 { 0x1B } else { 0x00 }
}

// Galois Field Multiplication for y in [[0..16]]
fn gfmul(x: u8, y: u8) -> u8 {
    let mut out = 0;
    let mut mask = x;

    for i in 0..4 {
        if y & (1 << i) != 0 {
            out ^= mask;
        }

        mask = xt2(x);
    }

    mask
}

fn aes32esmi(rs1: u32, rs2: u32, bs: u8) -> u32 {
    let shift_amount = bs * 8;

    // Substitution
    let sub_input = (rs2 >> shift_amount) & 0xFF;
    let sub_output = SBOX[sub_input as usize] as u8;

    // Mix Columns
    let mixed = u32::from_be_bytes([
        gfmul(sub_output, 0x3),
        sub_output,
        sub_output,
        gfmul(sub_output, 0x2),
    ]);

    // Add Roundkey
    rs1 ^ mixed.rotate_left(shift_amount)
}

fn aes32esi(rs1: u32, rs2: u32, bs: u8) -> u32 {
    let shift_amount = bs * 8;

    // Substitution
    let sub_input = (rs2 >> shift_amount) & 0xFF;
    let sub_output = SBOX[sub_input as usize] as u32;

    // Add Roundkey
    rs1 ^ (sub_output << shift_amount)
}
```

### Middle Round implementation

This can be used to implement an encryption middle encryption round, where `rk`
is an array of the roundkeys and `block` is the input state. Note, how in the
following code example it manually handles the shifting of rows.

```rust,no_run,no_playground
// Block and RoundKey contain little-endian encoded rows
let RoundKey(mut a0, mut a1, mut a2, mut a3) = rk[i];

a0 = aes32esmi(a0, block.0, 0);
a0 = aes32esmi(a0, block.1, 1);
a0 = aes32esmi(a0, block.2, 2);
a0 = aes32esmi(a0, block.3, 3);

a1 = aes32esmi(a1, block.1, 0);
a1 = aes32esmi(a1, block.2, 1);
a1 = aes32esmi(a1, block.3, 2);
a1 = aes32esmi(a1, block.0, 3);

a2 = aes32esmi(a2, block.2, 0);
a2 = aes32esmi(a2, block.3, 1);
a2 = aes32esmi(a2, block.0, 2);
a2 = aes32esmi(a2, block.1, 3);

a3 = aes32esmi(a3, block.3, 0);
a3 = aes32esmi(a3, block.0, 1);
a3 = aes32esmi(a3, block.1, 2);
a3 = aes32esmi(a3, block.2, 3);

block = Block(a0, a1, a2, a3);
```

### Final Round implementation

Similarly to the [Middle Round implementation](#middle-round-implementation),
the final round is implemented. Here, the `aes32esmi` instruction is replaced
by the `aes32esi` instruction.

```rust,no_run,no_playground
// Block and RoundKey contain little-endian encoded rows
let RoundKey(mut a0, mut a1, mut a2, mut a3) = rk[i];

a0 = aes32esi(a0, block.0, 0);
a0 = aes32esi(a0, block.1, 1);
a0 = aes32esi(a0, block.2, 2);
a0 = aes32esi(a0, block.3, 3);

a1 = aes32esi(a1, block.1, 0);
a1 = aes32esi(a1, block.2, 1);
a1 = aes32esi(a1, block.3, 2);
a1 = aes32esi(a1, block.0, 3);

a2 = aes32esi(a2, block.2, 0);
a2 = aes32esi(a2, block.3, 1);
a2 = aes32esi(a2, block.0, 2);
a2 = aes32esi(a2, block.1, 3);

a3 = aes32esi(a3, block.3, 0);
a3 = aes32esi(a3, block.0, 1);
a3 = aes32esi(a3, block.1, 2);
a3 = aes32esi(a3, block.2, 3);

block = Block(a0, a1, a2, a3);
```

## Decryption

```rust,no_run,no_playground

```

## Key Schedule implementation

To implement the key schedule, we can also use the `aes32esi` instruction. This
prevents the need for a substitution table in software. The implementation
differs slightly between AES128, AES196 and AES256 and therefore all three
implementations are given separately.

```rust,no_run,no_playground
pub struct AES128Key(u32, u32, u32, u32);
pub struct AES196Key(u32, u32, u32, u32, u32, u32);
pub struct AES256Key(u32, u32, u32, u32, u32, u32, u32, u32);

pub struct RoundKey(u32, u32, u32, u32);

fn aes128_key_schedule(ck: AES128Key) -> [RoundKey; 11] {
    let mut rk = [0u32; 11 * 4];

    let AES128Key(
        mut t0, mut t1,
        mut t2, mut t3,
    ) = ck;

    let mut i = 0;
    loop {
        rk[(i << 2) + 0] = t0;
        rk[(i << 2) + 1] = t1;
        rk[(i << 2) + 2] = t2;
        rk[(i << 2) + 3] = t3;

        if i == 10 {
            break;
        }

        t0 ^= u32::from(RCON[i]);
        let tr = t3.rotate_right(8);

        t0 = aes32esi(t0, tr, 0);
        t0 = aes32esi(t0, tr, 1);
        t0 = aes32esi(t0, tr, 2);
        t0 = aes32esi(t0, tr, 3);

        t1 ^= t0;
        t2 ^= t1;
        t3 ^= t2;

        i += 1;
    }

    // SAFETY: We know that rk has 13 * 4 times a u32. So it has space for 13 RoundKeys
    unsafe { core::mem::transmute(rk) }
}

fn aes196_key_schedule(ck: AES196Key) -> [RoundKey; 13] {
    let mut rk = [0u32; 13 * 4];

    let AES196Key(
        mut t0, mut t1,
        mut t2, mut t3,
        mut t4, mut t5,
    ) = ck;

    let mut i = 0;
    loop {
        rk[i * 6 + 0] = t0;
        rk[i * 6 + 1] = t1;
        rk[i * 6 + 2] = t2;
        rk[i * 6 + 3] = t3;

        if i == 8 {
            break;
        }

        rk[i * 6 + 4] = t4;
        rk[i * 6 + 5] = t5;

        t0 ^= u32::from(RCON[i]);
        let tr = t5.rotate_right(8);

        t0 = aes32esi(t0, tr, 0);
        t0 = aes32esi(t0, tr, 1);
        t0 = aes32esi(t0, tr, 2);
        t0 = aes32esi(t0, tr, 3);

        t1 ^= t0;
        t2 ^= t1;
        t3 ^= t2;
        t4 ^= t3;
        t5 ^= t4;

        i += 1;
    }

    // SAFETY: We know that rk has 13 * 4 times a u32. So it has space for 13 RoundKeys
    unsafe { core::mem::transmute(rk) }
}

fn aes256_key_schedule(ck: AES256Key) -> [RoundKey; 15] {
    let mut rk = [0u32; 15 * 4];

    let AES256Key(
        mut t0, mut t1,
        mut t2, mut t3,
        mut t4, mut t5,
        mut t6, mut t7,
    ) = ck;

    let mut i = 0;
    loop {
        rk[i * 8 + 0] = t0;
        rk[i * 8 + 1] = t1;
        rk[i * 8 + 2] = t2;
        rk[i * 8 + 3] = t3;

        if i == 7 {
            break;
        }

        rk[i * 8 + 4] = t4;
        rk[i * 8 + 5] = t5;
        rk[i * 8 + 6] = t6;
        rk[i * 8 + 7] = t7;

        t0 ^= u32::from(RCON[i]);
        let tr = t7.rotate_right(8);

        t0 = aes32esi(t0, tr, 0);
        t0 = aes32esi(t0, tr, 1);
        t0 = aes32esi(t0, tr, 2);
        t0 = aes32esi(t0, tr, 3);

        t1 ^= t0;
        t2 ^= t1;
        t3 ^= t2;

        t4 = aes32esi(t4, t3, 0);
        t4 = aes32esi(t4, t3, 1);
        t4 = aes32esi(t4, t3, 2);
        t4 = aes32esi(t4, t3, 3);

        t5 ^= t4;
        t6 ^= t5;
        t7 ^= t6;

        i += 1;
    }

    // SAFETY: We know that rk has 15 * 4 times a u32. So it has space for 15 RoundKeys
    unsafe { core::mem::transmute(rk) }
}

fn aes_decrypt_key_schedule<const KEYS: usize>(rk: &mut [RoundKey; KEYS]) {
    fn subkey(mut x: u32) -> u32 {
        let mut y;

        unsafe {
            y = aes32esi(0, x, 0);
            y = aes32esi(y, x, 1);
            y = aes32esi(y, x, 2);
            y = aes32esi(y, x, 3);

            x = aes32dsmi(0, y, 0);
            x = aes32dsmi(x, y, 1);
            x = aes32dsmi(x, y, 2);
            x = aes32dsmi(x, y, 3);
        }

        x
    }

    for k in &mut rk[1..KEYS - 1] {
        unsafe {
            k.0 = subkey(k.0);
            k.1 = subkey(k.1);
            k.2 = subkey(k.2);
            k.3 = subkey(k.3);
        }
    }
}

fn aes128_decrypt_key_schedule(rk: &mut [RoundKey; 11]) {
    aes_decrypt_key_schedule::<11>(rk)
}

fn aes196_decrypt_key_schedule(rk: &mut [RoundKey; 13]) {
    aes_decrypt_key_schedule::<13>(rk)
}

fn aes256_decrypt_key_schedule(rk: &mut [RoundKey; 15]) {
    aes_decrypt_key_schedule::<15>(rk)
}
```

[^1]: <https://www.youtube.com/watch?v=-HVRjbxWF-I>

[AES]: https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
[Sail]: https://github.com/riscv/sail-riscv/tree/master/riscv/sail-riscv/model/riscv_types_kext.sail
