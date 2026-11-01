use crate::{print_uart0, u64_to_hex};

pub fn init_trap() {
    unsafe {
        let base = trap_handler_wrapper as *const () as u64;
        debug_assert!(
            base & 0b11 == 0,
            "trap_handler_wrapper must be aligned to 4 bytes"
        );
        let mode = 0b00; // Direct mode
        let value = base | mode;
        core::arch::asm!("csrw stvec, {}", in(reg) value);
    }
}

#[unsafe(no_mangle)]
extern "C" fn trap_handler() {
    let sepc: u64;
    let scause: u64;
    let stval: u64;
    unsafe {
        core::arch::asm!("csrr {}, sepc", out(reg) sepc);
        core::arch::asm!("csrr {}, scause", out(reg) scause);
        core::arch::asm!("csrr {}, stval", out(reg) stval);
    }
    unsafe {
        print_uart0(b"trap_handler:\n  sepc=");
        print_uart0(&u64_to_hex(sepc));
        print_uart0(b"\n  scause=");
        print_uart0(&u64_to_hex(scause));
        print_uart0(b"\n  stval=");
        print_uart0(&u64_to_hex(stval));
        print_uart0(b"\n");
    }
    unsafe {
        if scause >> 63 == 1 {
            print_uart0(b"  interrupt\n");
        } else {
            print_uart0(b"  exception\n");
            core::arch::asm!("csrw sepc, {}", in(reg) sepc + 4);
        }
    }
}

unsafe extern "C" {
    fn trap_handler_wrapper() -> !;
}

core::arch::global_asm!(
    r##"
    .section .text
    .globl trap_handler_wrapper
    .type trap_handler_wrapper, @function
    .align 2 // stvec のダイレクトモードを使うので 4 バイト境界で OK
trap_handler_wrapper:
    addi sp, sp, -256 // スタックは 16 バイト境界
    sd  ra,   0(sp)
    addi ra, sp, 256
    sd  ra,   8(sp)  // 元の sp
    sd  gp,  16(sp)
    sd  tp,  24(sp)
    sd  t0,  32(sp)
    sd  t1,  40(sp)
    sd  t2,  48(sp)
    sd  s0,  56(sp)
    sd  s1,  64(sp)
    sd  a0,  72(sp)
    sd  a1,  80(sp)
    sd  a2,  88(sp)
    sd  a3,  96(sp)
    sd  a4, 104(sp)
    sd  a5, 112(sp)
    sd  a6, 120(sp)
    sd  a7, 128(sp)
    sd  s2, 136(sp)
    sd  s3, 144(sp)
    sd  s4, 152(sp)
    sd  s5, 160(sp)
    sd  s6, 168(sp)
    sd  s7, 176(sp)
    sd  s8, 184(sp)
    sd  s9, 192(sp)
    sd s10, 200(sp)
    sd s11, 208(sp)
    sd  t3, 216(sp)
    sd  t4, 224(sp)
    sd  t5, 232(sp)
    sd  t6, 240(sp)

    call trap_handler

    ld  ra,   0(sp)
    // sp は復元不要
    ld  gp,  16(sp)
    ld  tp,  24(sp)
    ld  t0,  32(sp)
    ld  t1,  40(sp)
    ld  t2,  48(sp)
    ld  s0,  56(sp)
    ld  s1,  64(sp)
    ld  a0,  72(sp)
    ld  a1,  80(sp)
    ld  a2,  88(sp)
    ld  a3,  96(sp)
    ld  a4, 104(sp)
    ld  a5, 112(sp)
    ld  a6, 120(sp)
    ld  a7, 128(sp)
    ld  s2, 136(sp)
    ld  s3, 144(sp)
    ld  s4, 152(sp)
    ld  s5, 160(sp)
    ld  s6, 168(sp)
    ld  s7, 176(sp)
    ld  s8, 184(sp)
    ld  s9, 192(sp)
    ld s10, 200(sp)
    ld s11, 208(sp)
    ld  t3, 216(sp)
    ld  t4, 224(sp)
    ld  t5, 232(sp)
    ld  t6, 240(sp)
    addi sp, sp, 256

    sret
    "##
);
