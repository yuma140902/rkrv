use core::cell::UnsafeCell;

pub static NR_HARTS: usize = 1;

pub const fn get_hart_id() -> usize {
    0
}

struct PerHart {
    pub current_tid: UnsafeCell<usize>,
    pub round_robin_counter: UnsafeCell<usize>,
}

static PER_HARTS: [PerHart; NR_HARTS] = [PerHart {
    current_tid: UnsafeCell::new(0),
    round_robin_counter: UnsafeCell::new(0),
}];

unsafe impl Sync for PerHart {}

pub fn read_current_tid() -> usize {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].current_tid.get() }
}

pub fn write_current_tid(tid: usize) {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].current_tid.get() = tid }
}

pub fn read_round_robin_counter() -> usize {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].round_robin_counter.get() }
}

pub fn write_round_robin_counter(counter: usize) {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].round_robin_counter.get() = counter }
}
