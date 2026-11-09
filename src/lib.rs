#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![no_std]
#![feature(riscv_ext_intrinsics)]
#![feature(alloc_error_handler)]

pub mod alloc;
pub mod per_hart;
pub mod sbi;
pub mod sched;
pub mod trap;

unsafe extern "C" {
    pub static __bss_start: u8;
    pub static __bss_end: u8;
}

/// # Safety
///
/// QEMU virt machine で実行していること
pub unsafe fn print_uart0(s: &[u8]) {
    for &c in s {
        // SAFETY: QEMU virt machine は 0x1000_0000 に UART0 がマップされている
        unsafe {
            core::ptr::write_volatile(0x1000_0000 as *mut u8, c);
        }
    }
}

/// # Safety
///
/// TODO
pub unsafe fn print_uart0_hex_line(s: &str, n: usize) {
    unsafe {
        print_uart0(s.as_bytes());
        print_uart0(b": ");
        print_uart0(&usize_to_hex(n));
        print_uart0(b"\n");
    }
}

pub fn usize_to_hex(n: usize) -> [u8; 18] {
    const {
        assert!(core::mem::size_of::<usize>() == 8);
    }
    let mut buf = [0u8; 18];
    buf[0] = b'0';
    buf[1] = b'x';
    for i in 0..16 {
        let shift = (15 - i) * 4;
        let digit = ((n >> shift) & 0xf) as u8;
        buf[i + 2] = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + (digit - 10)
        };
    }
    buf
}
