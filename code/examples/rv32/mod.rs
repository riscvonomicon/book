#[macro_export]
macro_rules! assert {
    ($condition:expr) => {{
        if ! { $condition } {
            $crate::rv32::print(file!());
            $crate::rv32::print(":");
            $crate::rv32::print_decimal(line!());
            $crate::rv32::print(": Assertion failed \"");
            $crate::rv32::print(stringify!($condition));
            $crate::rv32::print("\"\n");
            $crate::rv32::exit(1);
        }
    }};
}

pub fn print_buf(buf: &[u8]) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in ("a7") 64,
            in ("a0") 1,
            in ("a1") buf as *const [u8] as *const u8,
            in ("a2") buf.len(),
        );
    }
}

pub fn print(s: &str) {
    print_buf(s.as_bytes())
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}

pub fn print_decimal(mut num: u32) {
    if num == 0 {
        print("0");
    }

    const MAX_DIGITS: usize = 10;

    let num_digits = u32::from(num % 10 != 0) + num.ilog10();

    let mut buf = [0u8; MAX_DIGITS];

    for i in 0..num_digits {
        buf[(num_digits - i - 1) as usize] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    print_buf(&buf);
}

pub fn print_u128(num: u128) {
    for i in 0..4 {
        if i != 0 {
            print(" ");
        }

        print_hex(((num >> ((3-i)*32)) & 0xFFFF_FFFF) as u32);
    }
}

pub fn print_hex(num: u32) {
    const LUT: [&str; 16] = [
        "0", 
        "1", 
        "2", 
        "3", 
        "4", 
        "5", 
        "6", 
        "7", 
        "8", 
        "9", 
        "A", 
        "B", 
        "C", 
        "D", 
        "E", 
        "F", 
    ];

    for i in 0..8 {
        if i == 4 {
            print("_");
        }

        print(LUT[((num >> ((7-i)*4)) & 0xF) as usize]);
    }
}

pub fn exit(status_code: u32) -> ! {
    unsafe {
        core::arch::asm!(
            "ecall",
            in ("a7") 93,
            in ("a0") status_code,
            options (noreturn)
        );
    }
}

#[macro_export]
macro_rules! entry {
    ($entry_fn:ident) => {
        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            if let Some(loc) = info.location() {
                $crate::rv32::print("Code paniced at ");
                $crate::rv32::print(loc.file());
                $crate::rv32::print(":");
                $crate::rv32::print_decimal(loc.line());
                $crate::rv32::print(":");
                $crate::rv32::print_decimal(loc.column());
                $crate::rv32::print("\n");
            } else {
                $crate::rv32::print("Code paniced\n");
            }

            $crate::rv32::exit(1)
        }

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            $entry_fn();

            $crate::rv32::exit(0)
        }
    }
}