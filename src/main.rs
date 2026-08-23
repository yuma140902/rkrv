#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}
