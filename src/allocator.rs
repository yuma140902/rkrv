use crate::sv39::{self, NonNullPa};

pub struct PageAllocator {
    free_list_head: Option<NonNullPa>,
}

struct FreeListNode {
    next: Option<NonNullPa>,
}

const _: () = assert!(core::mem::size_of::<FreeListNode>() < sv39::PAGE_SIZE);

impl PageAllocator {
    /// # Safety
    ///
    /// base_addr と last_addr_exclusive の間の範囲が未使用であること
    pub unsafe fn new(base_addr: NonNullPa, last_addr_exclusive: NonNullPa) -> Self {
        assert!(
            base_addr
                .get()
                .is_multiple_of(core::mem::align_of::<FreeListNode>())
        );
        assert!(base_addr.get().is_multiple_of(sv39::PAGE_SIZE));
        assert!(last_addr_exclusive.get().is_multiple_of(sv39::PAGE_SIZE));
        assert!(base_addr < last_addr_exclusive);
        let mut current = base_addr;
        while current < last_addr_exclusive {
            let next = current + sv39::PAGE_SIZE;
            if next >= last_addr_exclusive {
                break;
            }
            unsafe {
                core::ptr::write(
                    current.get() as *mut FreeListNode,
                    FreeListNode { next: Some(next) },
                );
            }
            current = next;
        }
        unsafe {
            core::ptr::write(
                current.get() as *mut FreeListNode,
                FreeListNode { next: None },
            );
        }
        Self {
            free_list_head: Some(base_addr),
        }
    }

    pub const fn alloc_page(&mut self) -> Option<NonNullPa> {
        if let Some(page) = self.free_list_head {
            unsafe {
                let node = &*(page.get() as *const FreeListNode);
                self.free_list_head = node.next;
            }
            Some(page)
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// - page は alloc_page で返却されたページであること
    /// - page を誰も参照していないこと
    /// - page ごとに 1 度だけ呼び出すこと
    pub const unsafe fn dealloc_page(&mut self, page: NonNullPa) {
        unsafe {
            let node = &mut *(page.get() as *mut FreeListNode);
            node.next = self.free_list_head;
        }
        self.free_list_head = Some(page);
    }
}
