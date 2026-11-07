use core::sync::atomic::AtomicU64;

use crate::print_uart0_hex_line;

#[global_allocator]
pub static KALLOC: KAlloc = KAlloc::uninitialized();

pub struct KAlloc {
    base: AtomicU64,
    current: AtomicU64,
}

impl KAlloc {
    const fn uninitialized() -> Self {
        Self {
            base: AtomicU64::new(0),
            current: AtomicU64::new(0),
        }
    }
}

#[alloc_error_handler]
fn on_alloc_error(_l: core::alloc::Layout) -> ! {
    unsafe {
        core::arch::asm!("ebreak");
        loop {
            core::arch::riscv64::wfi();
        }
    }
}

unsafe impl core::alloc::GlobalAlloc for KAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe {
            print_uart0_hex_line("ALLOC size ", layout.size());
            print_uart0_hex_line("ALLOC align", layout.align());
        }
        let size = layout.size() as u64;
        let align = layout.align() as u64;
        let current = self.current.load(core::sync::atomic::Ordering::SeqCst);

        for i in 0.. {
            if (current + i).is_multiple_of(align) {
                unsafe {
                    print_uart0_hex_line("ALLOC found aligned address", (current + i) as usize);
                }
                let new_current = current + i + size;
                self.current
                    .store(new_current, core::sync::atomic::Ordering::SeqCst);
                return (current + i) as *mut u8;
            }
        }
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // TODO:
    }
}

/// # Safety
///
/// TODO
pub unsafe fn init(base: u64) {
    use core::sync::atomic::Ordering::SeqCst;
    KALLOC.base.store(base, SeqCst);
    KALLOC.current.store(base, SeqCst);
}
