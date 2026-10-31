use core::cell::UnsafeCell;

pub const STACK_SIZE: usize = 16 * 1024;

#[repr(C)]
pub struct Task {
    sp: UnsafeCell<u64>,
    stack: UnsafeCell<TaskStack>,
}

#[repr(C, align(16))]
pub struct TaskStack([u8; STACK_SIZE]);

#[repr(C)]
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

pub const NR_TASKS: usize = 2;
static mut TASKS: [Task; NR_TASKS] = [
    Task {
        sp: UnsafeCell::new(0),
        stack: UnsafeCell::new(TaskStack([0; STACK_SIZE])),
    },
    Task {
        sp: UnsafeCell::new(0),
        stack: UnsafeCell::new(TaskStack([0; STACK_SIZE])),
    },
];

pub fn init_task(tid: usize, entry: extern "C" fn()) {
    if tid >= NR_TASKS {
        panic!(
            "Task ID {} exceeds the maximum number of tasks {}",
            tid, NR_TASKS
        );
    }
    let task = unsafe { &mut TASKS[tid] };
    let stack_top = task.stack.get() as usize + STACK_SIZE;
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
        ra: entry as usize as u64,
    };
    let context_ptr = (stack_top - core::mem::size_of::<Context>()) as *mut Context;
    unsafe {
        core::ptr::write(context_ptr, context);
    }
    task.sp = UnsafeCell::new(context_ptr as u64);
}

pub fn switch_tasks(current_tid: usize, next_tid: usize) {
    let current = unsafe { &TASKS[current_tid] };
    let next = unsafe { &TASKS[next_tid] };
    unsafe {
        swtch(current.sp.get(), next.sp.get());
    }
}

pub fn load_task_context(tid: usize) -> ! {
    let task = unsafe { &TASKS[tid] };
    unsafe {
        load_context(task.sp.get());
    }
}

core::arch::global_asm!(
    r##"
    .section .text
    .globl swtch
    .type swtch, @function
    .align 2
swtch:
    // a0: current: &mut Task->sp, a1: next: &Task->sp

    addi sp, sp, -104
    sd s0, 0(sp)
    sd s1, 8(sp)
    sd s2, 16(sp)
    sd s3, 24(sp)
    sd s4, 32(sp)
    sd s5, 40(sp)
    sd s6, 48(sp)
    sd s7, 56(sp)
    sd s8, 64(sp)
    sd s9, 72(sp)
    sd s10, 80(sp)
    sd s11, 88(sp)
    sd ra, 96(sp)
    sd sp, 0(a0)

    ld sp, 0(a1)
1:
    ld s0, 0(sp)
    ld s1, 8(sp)
    ld s2, 16(sp)
    ld s3, 24(sp)
    ld s4, 32(sp)
    ld s5, 40(sp)
    ld s6, 48(sp)
    ld s7, 56(sp)
    ld s8, 64(sp)
    ld s9, 72(sp)
    ld s10, 80(sp)
    ld s11, 88(sp)
    ld ra, 96(sp)
    addi sp, sp, 104
    ret


    .globl load_context
    .type load_context, @function
    .align 2
load_context:
    // a0: &mut Task->sp
    ld sp, 0(a0)
    j 1b
    "##
);

unsafe extern "C" {
    fn swtch(current: *mut u64, next: *const u64);
    fn load_context(task_sp: *const u64) -> !;
}
