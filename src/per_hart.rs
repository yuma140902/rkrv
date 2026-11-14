use core::cell::UnsafeCell;

pub static NR_HARTS: usize = 1;

pub const fn get_hart_id() -> usize {
    0
}

struct PerHart {
    pub current_tid: UnsafeCell<usize>,
    pub round_robin_counter: UnsafeCell<usize>,
    pub interrupt_disable_counter: UnsafeCell<usize>,
    pub original_sie_bit: UnsafeCell<bool>,
}

static PER_HARTS: [PerHart; NR_HARTS] = [PerHart {
    current_tid: UnsafeCell::new(0),
    round_robin_counter: UnsafeCell::new(0),
    interrupt_disable_counter: UnsafeCell::new(0),
    original_sie_bit: UnsafeCell::new(true),
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

pub fn read_interrupt_disable_counter() -> usize {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].interrupt_disable_counter.get() }
}

pub fn write_interrupt_disable_counter(counter: usize) {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].interrupt_disable_counter.get() = counter }
}

pub fn read_original_sie_bit() -> bool {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].original_sie_bit.get() }
}

pub fn write_original_sie_bit(bit: bool) {
    let this_hart = get_hart_id();
    unsafe { *PER_HARTS[this_hart].original_sie_bit.get() = bit }
}
