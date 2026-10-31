#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

use rkrv::{print_uart0, sched, usize_to_hex};

core::arch::global_asm!(
    r#"
    .section .text.entry
    .globl _start
    .type _start, @function
_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    la sp, boot_stack_top
    .option pop

    la t0, __bss_start
    la t1, __bss_end
1:
    bgeu t0, t1, 2f
    sd zero, 0(t0)
    addi t0, t0, 8
    j 1b

2:
    call rust_main

3:
    wfi
    j 3b

    .section .bss.stack, "aw", @nobits
    .align 12
boot_stack:
    .space 16384
boot_stack_top:
"#
);

unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
}

#[unsafe(no_mangle)]
extern "C" fn rust_main(_hart_id: usize, _dtb_address: usize) -> ! {
    unsafe {
        print_uart0(b"rkrv rust_main\n");

        print_uart0(b"__bss_start: ");
        print_uart0(&usize_to_hex((&raw const __bss_start) as usize));
        print_uart0(b"\n");

        print_uart0(b"__bss_end: ");
        print_uart0(&usize_to_hex((&raw const __bss_end) as usize));
        print_uart0(b"\n");
    }
    sched::init_task(0, task1);
    sched::init_task(1, task2);
    sched::load_task_context(0);
    #[allow(unreachable_code)]
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}

extern "C" fn task1() {
    unsafe {
        loop {
            print_uart0(b"task1 start\n");
            sched::switch_tasks(0, 1);
        }
    }
}

extern "C" fn task2() {
    unsafe {
        loop {
            print_uart0(b"task2 start\n");
            sched::switch_tasks(1, 0);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}
