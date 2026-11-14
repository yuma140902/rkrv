use core::cell::UnsafeCell;

use crate::{
    allocator::page::FreeListPageAllocator,
    sv39::{self, NonNullPa},
};

#[global_allocator]
pub static ALLOC: RkrvGlobalAlloc = RkrvGlobalAlloc::uninitialized();

pub struct RkrvGlobalAlloc {
    page_allocator: UnsafeCell<FreeListPageAllocator>,
}

unsafe impl Sync for RkrvGlobalAlloc {}

impl RkrvGlobalAlloc {
    const fn uninitialized() -> Self {
        Self {
            page_allocator: UnsafeCell::new(FreeListPageAllocator::new_empty()),
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

unsafe impl core::alloc::GlobalAlloc for RkrvGlobalAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.size() > sv39::PAGE_SIZE {
            return core::ptr::null_mut();
        }
        if layout.align() > sv39::PAGE_SIZE {
            return core::ptr::null_mut();
        }
        unsafe {
            let p = &mut *self.page_allocator.get();
            p.alloc_page()
                .map_or(core::ptr::null_mut(), |page| page.get() as *mut u8)
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe {
            let p = &mut *self.page_allocator.get();
            let page = NonNullPa::new(_ptr as usize).unwrap();
            p.dealloc_page(page);
        }
    }
}

/// # Safety
///
/// TODO
pub unsafe fn init(base_addr: NonNullPa, last_addr_exclusive: NonNullPa) {
    unsafe {
        core::ptr::write(
            ALLOC.page_allocator.get(),
            FreeListPageAllocator::new(base_addr, last_addr_exclusive),
        );
    }
}
