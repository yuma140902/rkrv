use core::cell::UnsafeCell;

pub const STACK_SIZE: usize = 16 * 1024;

#[repr(C)]
pub struct Task {
    sp: UnsafeCell<u64>,
    entry: extern "C" fn(),
}

#[repr(C, align(16))]
pub struct TaskStack([u8; STACK_SIZE]);

#[repr(C, align(16))]
pub struct Context {
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
    ra: u64,
}
const _: () = assert!(core::mem::size_of::<Context>() == 112);

pub const NR_TASKS: usize = 4;
static mut TASKS: [Option<Task>; NR_TASKS] = [const { None }; NR_TASKS];
static mut STACKS: [TaskStack; NR_TASKS] = [const { TaskStack([0u8; STACK_SIZE]) }; NR_TASKS];
static mut CURRENT_TID: usize = 0;
static mut ROUND_ROBIN_COUNTER: usize = 0;

pub fn init_task(tid: usize, entry: extern "C" fn()) {
    if tid >= NR_TASKS {
        panic!(
            "Task ID {} exceeds the maximum number of tasks {}",
            tid, NR_TASKS
        );
    }
    let task_slot = unsafe { &mut TASKS[tid] };
    let task = task_slot.insert(Task {
        sp: UnsafeCell::new(0),
        entry,
    });
    let stack_top = unsafe { (&raw mut STACKS[tid]) as usize + STACK_SIZE };
    let context = Context {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
        s4: 0,
        s5: 0,
        s6: 0,
        s7: 0,
        s8: 0,
        s9: 0,
        s10: 0,
        s11: 0,
        ra: task_trampoline as *const () as u64,
    };
    let context_ptr = (stack_top - core::mem::size_of::<Context>()) as *mut Context;
    unsafe {
        core::ptr::write(context_ptr, context);
    }
    task.sp = UnsafeCell::new(context_ptr as u64);
}

fn pick_next_task(current_tid: usize) -> Option<usize> {
    for i in 0..NR_TASKS {
        let tid = (unsafe { ROUND_ROBIN_COUNTER } + i) % NR_TASKS;
        if tid == current_tid {
            continue;
        }
        if unsafe { TASKS[tid].is_some() } {
            unsafe {
                ROUND_ROBIN_COUNTER = (tid + 1) % NR_TASKS;
            }
            return Some(tid);
        }
    }
    None
}

pub fn yield_task() {
    let current_tid = unsafe { CURRENT_TID };
    let next_tid = match pick_next_task(current_tid) {
        Some(tid) => tid,
        None => return,
    };

    unsafe {
        assert!(next_tid != current_tid);
        switch_tasks(current_tid, next_tid);
    }
}

unsafe fn on_context_switch_pre(next_tid: usize) {
    unsafe {
        crate::trap::push_disable_interrupt();
        CURRENT_TID = next_tid;
    }
}

unsafe fn on_context_switch_post() {
    unsafe {
        crate::trap::pop_enable_interrupt();
    }
}

/// # Safety
///
/// - current_tid と next_tid が初期化済みのタスク ID であること
/// - current_tid と next_tid が同じでないこと
unsafe fn switch_tasks(current_tid: usize, next_tid: usize) {
    unsafe {
        on_context_switch_pre(next_tid);
        let current = TASKS[current_tid].as_ref().unwrap();
        let next = TASKS[next_tid].as_ref().unwrap();
        swtch(current.sp.get(), next.sp.get());
        on_context_switch_post();
    }
}

/// # Safety
///
/// - tid が初期化済みのタスク ID であること
pub unsafe fn load_task_context(tid: usize) -> ! {
    let task = unsafe { TASKS[tid].as_ref().unwrap() };
    unsafe {
        on_context_switch_pre(tid);
        load_context(task.sp.get());
    }
}

extern "C" fn task_trampoline() -> ! {
    unsafe {
        on_context_switch_post();
    }
    let entry = unsafe { TASKS[CURRENT_TID].as_ref().unwrap().entry };
    entry();
    loop {
        yield_task();
    }
}

unsafe extern "C" {
    fn swtch(current: *mut u64, next: *const u64);
    fn load_context(task_sp: *const u64) -> !;
}

core::arch::global_asm!(
    r##"
    .section .text
    .globl swtch
    .type swtch, @function
    .align 2
swtch:
    // a0: current: &mut Task->sp, a1: next: &Task->sp

    addi sp, sp, -112
    sd  s0,  0(sp)
    sd  s1,  8(sp)
    sd  s2, 16(sp)
    sd  s3, 24(sp)
    sd  s4, 32(sp)
    sd  s5, 40(sp)
    sd  s6, 48(sp)
    sd  s7, 56(sp)
    sd  s8, 64(sp)
    sd  s9, 72(sp)
    sd s10, 80(sp)
    sd s11, 88(sp)
    sd  ra, 96(sp)
    sd  sp,  0(a0)

    ld sp, 0(a1)
1:
    ld  s0,  0(sp)
    ld  s1,  8(sp)
    ld  s2, 16(sp)
    ld  s3, 24(sp)
    ld  s4, 32(sp)
    ld  s5, 40(sp)
    ld  s6, 48(sp)
    ld  s7, 56(sp)
    ld  s8, 64(sp)
    ld  s9, 72(sp)
    ld s10, 80(sp)
    ld s11, 88(sp)
    ld  ra, 96(sp)
    addi sp, sp, 112
    ret
    .size swtch, .-swtch

    .globl load_context
    .type load_context, @function
    .align 2
load_context:
    // a0: &mut Task->sp
    ld sp, 0(a0)
    j 1b
    .size load_context, .-load_context
    "##
);

#[cfg(test)]
mod tests {
    use super::*;

    fn context_switch_test() {
        init_task(0, task1);
        init_task(1, task2);
        unsafe {
            load_task_context(0);
        }
    }

    extern "C" fn task1() {
        unsafe {
            loop {
                crate::print_uart0(b"task1 start\n");
                yield_task();
            }
        }
    }

    extern "C" fn task2() {
        unsafe {
            loop {
                crate::print_uart0(b"task2 start\n");
                yield_task();
            }
        }
    }
}
