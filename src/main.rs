#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

core::arch::global_asm!(
    r#"
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

1:
    wfi
    j 1b

    .section .bss.stack, "aw", @nobits
    .align 12
boot_stack:
    .space 16384
boot_stack_top:
"#
);

#[unsafe(no_mangle)]
extern "C" fn rust_main(_hart_id: usize, _dtb_address: usize) -> ! {
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}
