use core::arch::riscv32::{aes32esi, aes32esmi};

#[derive(Clone, Copy)]
pub struct AES128Key(u32, u32, u32, u32);
#[derive(Clone)]
pub struct AES196Key(u32, u32, u32, u32, u32, u32);
#[derive(Clone)]
pub struct AES256Key(u32, u32, u32, u32, u32, u32, u32, u32);
#[derive(Clone, Copy)]
pub struct Block(u32, u32, u32, u32);
#[derive(Clone, Copy)]
pub struct RoundKey(u32, u32, u32, u32);

impl From<[u8; 16]> for AES128Key {
    fn from(v: [u8; 16]) -> Self {
        Self(
            u32::from_le_bytes([v[00], v[01], v[02], v[03]]),
            u32::from_le_bytes([v[04], v[05], v[06], v[07]]),
            u32::from_le_bytes([v[08], v[09], v[10], v[11]]),
            u32::from_le_bytes([v[12], v[13], v[14], v[15]]),
        )
    }
}

impl From<[u8; 24]> for AES196Key {
    fn from(v: [u8; 24]) -> Self {
        Self(
            u32::from_le_bytes([v[00], v[01], v[02], v[03]]),
            u32::from_le_bytes([v[04], v[05], v[06], v[07]]),
            u32::from_le_bytes([v[08], v[09], v[10], v[11]]),
            u32::from_le_bytes([v[12], v[13], v[14], v[15]]),
            u32::from_le_bytes([v[16], v[17], v[18], v[19]]),
            u32::from_le_bytes([v[20], v[21], v[22], v[23]]),
        )
    }
}

impl From<[u8; 32]> for AES256Key {
    fn from(v: [u8; 32]) -> Self {
        Self(
            u32::from_le_bytes([v[00], v[01], v[02], v[03]]),
            u32::from_le_bytes([v[04], v[05], v[06], v[07]]),
            u32::from_le_bytes([v[08], v[09], v[10], v[11]]),
            u32::from_le_bytes([v[12], v[13], v[14], v[15]]),
            u32::from_le_bytes([v[16], v[17], v[18], v[19]]),
            u32::from_le_bytes([v[20], v[21], v[22], v[23]]),
            u32::from_le_bytes([v[24], v[25], v[26], v[27]]),
            u32::from_le_bytes([v[28], v[29], v[30], v[31]]),
        )
    }
}

impl From<[u8; 16]> for Block {
    fn from(v: [u8; 16]) -> Self {
        Self(
            u32::from_le_bytes([v[00], v[01], v[02], v[03]]),
            u32::from_le_bytes([v[04], v[05], v[06], v[07]]),
            u32::from_le_bytes([v[08], v[09], v[10], v[11]]),
            u32::from_le_bytes([v[12], v[13], v[14], v[15]]),
        )
    }
}

impl Block {
    pub fn to_bytes(self) -> [u8; 16] {
        let Self(r0, r1, r2, r3) = self;

        let [r00, r01, r02, r03] = r0.to_le_bytes();
        let [r10, r11, r12, r13] = r1.to_le_bytes();
        let [r20, r21, r22, r23] = r2.to_le_bytes();
        let [r30, r31, r32, r33] = r3.to_le_bytes();

        [
            r00, r01, r02, r03, r10, r11, r12, r13, r20, r21, r22, r23, r30, r31, r32, r33,
        ]
    }
}

impl RoundKey {
    const ZERO: Self = Self(0, 0, 0, 0);

    pub fn to_bytes(self) -> [u8; 16] {
        let Self(r0, r1, r2, r3) = self;

        let [r00, r01, r02, r03] = r0.to_le_bytes();
        let [r10, r11, r12, r13] = r1.to_le_bytes();
        let [r20, r21, r22, r23] = r2.to_le_bytes();
        let [r30, r31, r32, r33] = r3.to_le_bytes();

        [
            r00, r01, r02, r03, r10, r11, r12, r13, r20, r21, r22, r23, r30, r31, r32, r33,
        ]
    }
}

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

#[target_feature(enable = "zkne")]
pub unsafe fn aes128_key_schedule(ck: AES128Key) -> [RoundKey; 11] {
    let mut rk = [0u32; 11 * 4];

    let AES128Key(mut t0, mut t1, mut t2, mut t3) = ck;

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

        // SAFETY: We know that the `zkne` extension is available
        unsafe {
            t0 = aes32esi(t0, tr, 0);
            t0 = aes32esi(t0, tr, 1);
            t0 = aes32esi(t0, tr, 2);
            t0 = aes32esi(t0, tr, 3);
        }

        t1 ^= t0;
        t2 ^= t1;
        t3 ^= t2;

        i += 1;
    }

    // SAFETY: We know that rk has 13 * 4 times a u32. So it has space for 13 RoundKeys
    unsafe { core::mem::transmute(rk) }
}

#[target_feature(enable = "zkne")]
pub unsafe fn aes196_key_schedule(ck: AES196Key) -> [RoundKey; 13] {
    let mut rk = [0u32; 13 * 4];

    let AES196Key(mut t0, mut t1, mut t2, mut t3, mut t4, mut t5) = ck;

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

        // SAFETY: We know that the `zkne` extension is available
        unsafe {
            t0 = aes32esi(t0, tr, 0);
            t0 = aes32esi(t0, tr, 1);
            t0 = aes32esi(t0, tr, 2);
            t0 = aes32esi(t0, tr, 3);
        }

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

#[target_feature(enable = "zkne")]
pub unsafe fn aes256_key_schedule(ck: AES256Key) -> [RoundKey; 15] {
    let mut rk = [0u32; 15 * 4];

    let AES256Key(mut t0, mut t1, mut t2, mut t3, mut t4, mut t5, mut t6, mut t7) = ck;

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

        // SAFETY: We know that the `zkne` extension is available
        unsafe {
            t0 = aes32esi(t0, tr, 0);
            t0 = aes32esi(t0, tr, 1);
            t0 = aes32esi(t0, tr, 2);
            t0 = aes32esi(t0, tr, 3);
        }

        t1 ^= t0;
        t2 ^= t1;
        t3 ^= t2;

        // SAFETY: We know that the `zkne` extension is available
        unsafe {
            t4 = aes32esi(t4, t3, 0);
            t4 = aes32esi(t4, t3, 1);
            t4 = aes32esi(t4, t3, 2);
            t4 = aes32esi(t4, t3, 3);
        }

        t5 ^= t4;
        t6 ^= t5;
        t7 ^= t6;

        i += 1;
    }

    // SAFETY: We know that rk has 15 * 4 times a u32. So it has space for 15 RoundKeys
    unsafe { core::mem::transmute(rk) }
}

#[target_feature(enable = "zkne")]
unsafe fn aes_enc<const KEYS: usize>(block: Block, rk: &[RoundKey; KEYS]) -> Block {
    let mut block = Block(
        block.0 ^ rk[0].0,
        block.1 ^ rk[0].1,
        block.2 ^ rk[0].2,
        block.3 ^ rk[0].3,
    );

    let mut a0;
    let mut a1;
    let mut a2;
    let mut a3;

    for i in 1..KEYS - 1 {
        a0 = rk[i].0;
        a1 = rk[i].1;
        a2 = rk[i].2;
        a3 = rk[i].3;

        // SAFETY: We know that the `zkne` extension is available
        unsafe {
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
        }

        block = Block(a0, a1, a2, a3);
    }

    a0 = rk[KEYS - 1].0;
    a1 = rk[KEYS - 1].1;
    a2 = rk[KEYS - 1].2;
    a3 = rk[KEYS - 1].3;

    // SAFETY: We know that the `zkne` extension is available
    unsafe {
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
    }

    Block(a0, a1, a2, a3)
}

#[target_feature(enable = "zkne")]
pub unsafe fn aes128(block: Block, rk: &[RoundKey; 11]) -> Block {
    // SAFETY: Same invariant as the parent function
    unsafe { aes_enc::<11>(block, rk) }
}

#[target_feature(enable = "zkne")]
pub unsafe fn aes196(block: Block, rk: &[RoundKey; 13]) -> Block {
    // SAFETY: Same invariant as the parent function
    unsafe { aes_enc::<13>(block, rk) }
}

#[target_feature(enable = "zkne")]
pub unsafe fn aes256(block: Block, rk: &[RoundKey; 15]) -> Block {
    // SAFETY: Same invariant as the parent function
    unsafe { aes_enc::<15>(block, rk) }
}