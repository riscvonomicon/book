#![no_std]
#![no_main]

use ralte32::{define_tests, println};

use riscvonomicon_code_examples::zk::aes32::{
    aes128, aes128_key_schedule, aes196, aes196_key_schedule, aes256, aes256_key_schedule,
    AES128Key, AES196Key, AES256Key, Block, RoundKey,
};

define_tests! {
    test_aes128,
    test_aes196,
    test_aes256,
}

fn test_aes128() {
    println!();
    let pt = 0x00112233445566778899aabbccddeeffu128;
    let ck = 0x000102030405060708090a0b0c0d0e0fu128;

    let ks = unsafe { aes128_key_schedule(AES128Key::from(ck.to_be_bytes())) };

    let ct = unsafe { aes128(Block::from(pt.to_be_bytes()), &ks) };

    println!("----- ROUND KEYS -----");
    for k in &ks {
        println!(u128::from_be_bytes(k.to_bytes()));
    }
    println!("-----  END KEYS  -----\n");

    println!(u128::from_be_bytes(ct.to_bytes()));
    println!();
}

fn test_aes196() {
    println!();
    let pt = 0x6bc1bee22e409f96e93d7e117393172au128;

    let ks = unsafe {
        aes196_key_schedule(AES196Key::from([
            0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90,
            0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b,
        ]))
    };

    let ct = unsafe { aes196(Block::from(pt.to_be_bytes()), &ks) };

    println!("----- ROUND KEYS -----");
    for k in &ks {
        println!(u128::from_be_bytes(k.to_bytes()));
    }
    println!("-----  END KEYS  -----\n");

    println!(u128::from_be_bytes(ct.to_bytes()));
    println!();
}

fn test_aes256() {
    println!();
    let pt = 0x6bc1bee22e409f96e93d7e117393172au128;

    let ks = unsafe {
        aes256_key_schedule(AES256Key::from([
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d,
            0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3,
            0x09, 0x14, 0xdf, 0xf4,
        ]))
    };

    let ct = unsafe { aes256(Block::from(pt.to_be_bytes()), &ks) };

    println!("----- ROUND KEYS -----");
    for k in &ks {
        println!(u128::from_be_bytes(k.to_bytes()));
    }
    println!("-----  END KEYS  -----\n");

    println!(u128::from_be_bytes(ct.to_bytes()));
    println!();
}
