extern crate alloc;

use core::{num::NonZeroUsize, ptr::NonNull};

use crate::{
    data::bitmap::Bitmap,
    sv39::{self, NonNullPa},
};

// size, log2, rank
//   4 KB, 12,  0
//   8 KB, 13,  1
//  16 KB, 14,  2
//  32 KB, 15,  3
//  64 KB, 16,  4
// 128 KB, 17,  5
// 256 KB, 18,  6
// 512 KB, 19,  7
//   1 MB, 20,  9
//   2 MB, 21, 10
//   4 MB, 22, 11
//   8 MB, 23, 12
//  16 MB, 24, 13
//  32 MB, 25, 14
//  64 MB, 26, 15
// 128 MB, 27, 16
// 256 MB, 28, 17
// 512 MB, 29, 18
//   1 GB, 30, 19

pub const NR_RANKS: usize = 20;

pub struct BuddyPageAllocator {
    aligned_base_addr: NonNullPa,
    free_bitmap: [Bitmap<NoOpAllocator>; NR_RANKS],
    free_lists: [Option<NonNull<FreeBlock>>; NR_RANKS],
    _aux_heap: alloc::boxed::Box<[u8], NoOpAllocator>,
}

struct FreeBlock {
    next: NonNull<Self>,
    prev: NonNull<Self>,
}

const _: () = assert!(core::mem::size_of::<FreeBlock>() < sv39::PAGE_SIZE);

const PAGE_SIZE_LOG2: usize = (sv39::PAGE_SIZE as u64).ilog2() as usize;

impl BuddyPageAllocator {
    fn split_page(page: NonNullPa, rank: usize) -> NonNullPa {
        let size = 1 << (rank + PAGE_SIZE_LOG2);
        page + size / 2
    }

    fn calc_min_rank(size: NonZeroUsize, align: NonZeroUsize) -> usize {
        let size = size.max(align);
        ((size.get() - 1).bit_width() as usize).saturating_sub(PAGE_SIZE_LOG2)
    }

    const fn get_buddy(page: NonNullPa, rank: usize) -> NonNullPa {
        let size = 1 << (rank + PAGE_SIZE_LOG2);
        let page_addr = page.as_usize();
        let buddy_addr = page_addr ^ size;
        NonNullPa::new(buddy_addr).unwrap()
    }

    fn is_free(&self, page: NonNullPa, rank: usize) -> bool {
        assert_eq!(page.as_usize() % (1 << (rank + PAGE_SIZE_LOG2)), 0);
        assert!(self.aligned_base_addr <= page);
        let page_offset = page.as_usize() - self.aligned_base_addr.as_usize();
        self.free_bitmap[rank][page_offset >> (rank + PAGE_SIZE_LOG2)]
    }

    fn set_free(&mut self, page: NonNullPa, rank: usize, free: bool) {
        assert_eq!(page.as_usize() % (1 << (rank + PAGE_SIZE_LOG2)), 0);
        assert!(self.aligned_base_addr <= page);
        let page_offset = page.as_usize() - self.aligned_base_addr.as_usize();
        self.free_bitmap[rank].set(page_offset >> (rank + PAGE_SIZE_LOG2), free);
    }

    unsafe fn add_to_free_list(&mut self, page: NonNullPa, rank: usize) {
        let new_node = NonNull::<FreeBlock>::with_exposed_provenance(page.as_nonzero_usize());
        let head = self.free_lists[rank];
        if let Some(head) = head {
            unsafe {
                // add new_node to the head of the list
                let tail = (*head.as_ptr()).prev;
                (*new_node.as_ptr()).next = head;
                (*new_node.as_ptr()).prev = tail;
                (*head.as_ptr()).prev = new_node;
                (*tail.as_ptr()).next = new_node;
            }
        } else {
            unsafe {
                self.free_lists[rank] = Some(new_node);
                (*new_node.as_ptr()).next = new_node;
                (*new_node.as_ptr()).prev = new_node;
            }
        }
    }

    unsafe fn remove_from_free_list(&mut self, page: NonNullPa, rank: usize) {
        // TODO: use NonNull::with_addr()
        let node = NonNull::<FreeBlock>::with_exposed_provenance(page.as_nonzero_usize());
        let head = self.free_lists[rank];
        if let Some(head) = head
            && head == node
        {
            if unsafe { (*node.as_ptr()).next } == node {
                // only one node in the list
                self.free_lists[rank] = None;
            } else {
                self.free_lists[rank] = unsafe { Some((*node.as_ptr()).next) };
            }
        }
        unsafe {
            let prev = (*node.as_ptr()).prev;
            let next = (*node.as_ptr()).next;
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).prev = prev;
        }
    }

    unsafe fn find_free_page(&self, rank: usize) -> Option<NonNullPa> {
        let current = self.free_lists[rank];
        let mut current = current?;
        loop {
            let page = NonNullPa::new(current.as_ptr() as usize).unwrap();
            if self.is_free(page, rank) {
                return Some(page);
            }
            current = unsafe { (*current.as_ptr()).next };
            if Some(current) == self.free_lists[rank] {
                break;
            }
        }
        None
    }

    unsafe fn alloc_from_rank(&mut self, rank: usize) -> Option<NonNullPa> {
        let page = unsafe { self.find_free_page(rank) }?;
        unsafe {
            self.remove_from_free_list(page, rank);
            self.set_free(page, rank, false);
        }
        Some(page)
    }

    /// # Safety
    ///
    /// TODO
    pub unsafe fn alloc(&mut self, size: NonZeroUsize, align: NonZeroUsize) -> Option<NonNullPa> {
        let rank = Self::calc_min_rank(size, align);
        if rank >= NR_RANKS {
            return None;
        }
        if let Some(page) = unsafe { self.alloc_from_rank(rank) } {
            return Some(page);
        }
        for higher_rank in (rank + 1)..NR_RANKS {
            if let Some(page) = unsafe { self.alloc_from_rank(higher_rank) } {
                // split the page into two pages of lower rank
                let mut current_rank = higher_rank;
                while current_rank > rank {
                    let buddy = Self::split_page(page, current_rank);
                    unsafe {
                        self.add_to_free_list(buddy, current_rank - 1);
                        self.set_free(buddy, current_rank - 1, true);
                    }
                    current_rank -= 1;
                }
                return Some(page);
            }
        }
        None
    }

    /// # Safety
    ///
    /// TODO
    pub unsafe fn dealloc(&mut self, page: NonNullPa, size: NonZeroUsize, align: NonZeroUsize) {
        let rank = Self::calc_min_rank(size, align);

        unsafe {
            self.set_free(page, rank, true);
            self.add_to_free_list(page, rank);
        }

        if rank >= NR_RANKS {
            return;
        }
        if rank + 1 >= NR_RANKS {
            return;
        }
        let mut page = page;
        for higher_rank in rank..(NR_RANKS - 1) {
            let buddy = Self::get_buddy(page, higher_rank);
            if !self.is_free(buddy, higher_rank) {
                break;
            }
            unsafe {
                self.remove_from_free_list(buddy, higher_rank);
                self.set_free(buddy, higher_rank, false);
                self.remove_from_free_list(page, higher_rank);
                self.set_free(page, higher_rank, false);
                page = if page < buddy { page } else { buddy };
                self.add_to_free_list(page, higher_rank + 1);
                self.set_free(page, higher_rank + 1, true);
            }
        }
    }

    /// # Safety
    ///
    /// TODO
    #[allow(clippy::boxed_local)]
    pub unsafe fn new(
        base_addr: NonNullPa,
        last_addr_exclusive: NonNullPa,
        mut aux_heap: alloc::boxed::Box<[u8], NoOpAllocator>,
    ) -> Self {
        assert!(base_addr.as_usize().is_multiple_of(sv39::PAGE_SIZE));
        assert!(
            last_addr_exclusive
                .as_usize()
                .is_multiple_of(sv39::PAGE_SIZE)
        );
        assert!(base_addr < last_addr_exclusive);

        let rounded_down_base_addr =
            NonNullPa::new(base_addr.as_usize() & !((1 << (PAGE_SIZE_LOG2 + NR_RANKS - 1)) - 1))
                .unwrap();
        let rounded_up_last_addr_exclusive = NonNullPa::new(
            (last_addr_exclusive.as_usize() + (1 << (PAGE_SIZE_LOG2 + NR_RANKS - 1)) - 1)
                & !((1 << (PAGE_SIZE_LOG2 + NR_RANKS - 1)) - 1),
        )
        .unwrap();
        let size = rounded_up_last_addr_exclusive.as_usize() - rounded_down_base_addr.as_usize();
        assert!(
            rounded_up_last_addr_exclusive
                .as_usize()
                .is_multiple_of(1 << (PAGE_SIZE_LOG2 + NR_RANKS - 1))
        );
        assert!(
            rounded_down_base_addr
                .as_usize()
                .is_multiple_of(1 << (PAGE_SIZE_LOG2 + NR_RANKS - 1))
        );
        assert!(size >= 1 << (PAGE_SIZE_LOG2 + NR_RANKS - 1));

        assert!(aux_heap.len() > 2 * size / sv39::PAGE_SIZE);

        let mut bump_index = 0;
        let free_bitmaps = core::array::from_fn(|rank| {
            let nr_pages = size / (1 << (rank + PAGE_SIZE_LOG2));
            let bitmap_size = nr_pages.div_ceil(8);
            let inner_box = unsafe {
                alloc::boxed::Box::from_non_null_in(
                    NonNull::from_mut(&mut aux_heap[bump_index..bump_index + bitmap_size]),
                    NoOpAllocator,
                )
            };
            bump_index += bitmap_size;
            Bitmap::new(inner_box, nr_pages)
        });

        let mut ret = Self {
            aligned_base_addr: rounded_down_base_addr,
            free_bitmap: free_bitmaps,
            free_lists: [None; NR_RANKS],
            aux_heap,
        };

        for page in (base_addr.as_usize()..last_addr_exclusive.as_usize()).step_by(sv39::PAGE_SIZE)
        {
            let page = NonNullPa::new(page).unwrap();
            unsafe {
                ret.dealloc(
                    page,
                    NonZeroUsize::new(sv39::PAGE_SIZE).unwrap(),
                    NonZeroUsize::new(sv39::PAGE_SIZE).unwrap(),
                );
            }
        }

        ret
    }
}

#[derive(Clone, Copy)]
pub struct NoOpAllocator;

unsafe impl core::alloc::Allocator for NoOpAllocator {
    fn allocate(
        &self,
        _layout: core::alloc::Layout,
    ) -> Result<NonNull<[u8]>, alloc::alloc::AllocError> {
        Err(alloc::alloc::AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: core::alloc::Layout) {
        // no op
    }
}
