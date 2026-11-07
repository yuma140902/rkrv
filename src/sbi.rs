pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

/// # Safety
///
/// TODO
#[allow(clippy::too_many_arguments)]
pub unsafe fn call_sbi(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    fid: u64,
    eid: u64,
) -> SbiRet {
    let error: i64;
    let value: u64;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") a0 => error,
            inlateout("a1") a1 => value,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error, value }
}

/// <https://docs.riscv.org/reference/sbi/v3.0/ext-legacy.html#5-1-2-extension-console-putchar-eid-0x01>
#[must_use]
pub fn legacy_console_putchar(c: u8) -> SbiRet {
    unsafe { call_sbi(c as u64, 0, 0, 0, 0, 0, 0, 1) }
}

/// <https://docs.riscv.org/reference/sbi/v3.0/ext-time.html#6-1-1-function-set-timer-fid-0>
#[must_use]
pub fn set_timer(stime_value: u64) -> SbiRet {
    unsafe { call_sbi(stime_value, 0, 0, 0, 0, 0, 0, 0x54494D45) }
}

#[must_use]
pub fn set_timer_relative(delta: u64) -> SbiRet {
    unsafe {
        let current: u64;
        core::arch::asm!("rdtime {}", out(reg) current);
        call_sbi(current + delta, 0, 0, 0, 0, 0, 0, 0x54494D45)
    }
}
