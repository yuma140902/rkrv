#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

extern crate alloc;

use rkrv::{print_uart0, print_uart0_hex_line, sbi, sched, trap};

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
    .size _start, .-_start

    .section .bss.stack, "aw", @nobits
    .align 12
boot_stack:
    .space 16384
boot_stack_top:
"#
);

#[unsafe(no_mangle)]
extern "C" fn rust_main(_hart_id: usize, _dtb_address: usize) -> ! {
    unsafe {
        print_uart0(b"rkrv rust_main\n");
        print_uart0_hex_line("__bss_start", (&raw const rkrv::__bss_start) as usize);
        print_uart0_hex_line("__bss_end  ", (&raw const rkrv::__bss_end) as usize);
    }
    unsafe {
        rkrv::alloc::init((&raw const rkrv::__bss_end) as u64);
    }
    trap::init_trap();

    let v = alloc::vec![1, 2, 3, 4, 5];
    for vi in v {
        unsafe {
            print_uart0_hex_line("vec element", vi);
        }
    }

    unsafe {
        // タイマー割り込みを有効化
        core::arch::asm!("csrrs zero, sie, {}", in(reg) 1_u64 << 5);
        // 割り込みを有効化
        core::arch::asm!("csrrs zero, sstatus, {}", in(reg) 1_u64 << 1);
    }

    let tid = sched::new_task(rust_main_task);
    assert!(tid == Some(0));
    unsafe {
        sched::load_task_context(0);
    }

    #[allow(unreachable_code)]
    loop {
        unsafe { core::arch::asm!("ebreak") }
    }
}

extern "C" fn rust_main_task() {
    unsafe {
        print_uart0(b"rkrv rust_main_task\n");
        let ret = sbi::set_timer_relative(10000000);
        loop {
            core::arch::asm!("wfi");
        }
        assert!(ret.error == 0);
        sched::new_task(task1).unwrap();
        sched::new_task(task2).unwrap();
        sched::new_task(task3).unwrap();
        loop {
            print_uart0(b"main\n");
            sched::yield_task();
        }
        #[allow(unreachable_code)]
        loop {
            core::arch::asm!("ebreak");
        }
    }
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }

    extern "C" fn task1() {
        unsafe {
            loop {
                print_uart0(b"1\n");
                sched::yield_task();
            }
        }
    }
    extern "C" fn task2() {
        unsafe {
            loop {
                print_uart0(b"2\n");
                sched::yield_task();
            }
        }
    }
    extern "C" fn task3() {
        unsafe {
            loop {
                print_uart0(b"3\n");
                sched::yield_task();
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        unsafe { core::arch::riscv64::wfi() }
    }
}
