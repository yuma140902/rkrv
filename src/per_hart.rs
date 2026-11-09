pub static NR_HARTS: usize = 1;

pub const fn get_hart_id() -> usize {
    0
}

pub struct PerHart {
    pub current_tid: usize,
    pub round_robin_counter: usize,
    pub interrupt_disable_counter: usize,
    pub original_sie_bit: bool,
}

pub static mut PER_HARTS: [PerHart; NR_HARTS] = [PerHart {
    current_tid: 0,
    round_robin_counter: 0,
    interrupt_disable_counter: 0,
    original_sie_bit: true,
}];

/// # Safety
///
/// TODO
pub unsafe fn get_per_hart() -> &'static mut PerHart {
    let hart_id = get_hart_id();
    unsafe { &mut PER_HARTS[hart_id] }
}
