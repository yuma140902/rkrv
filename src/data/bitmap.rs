extern crate alloc;

use alloc::boxed::Box;

pub struct Bitmap<A: alloc::alloc::Allocator = alloc::alloc::Global> {
    inner: Box<[u8], A>,
    len: usize,
}

impl<A: alloc::alloc::Allocator> Bitmap<A> {
    pub fn new(mut inner: Box<[u8], A>, len: usize) -> Self {
        assert!(
            len <= inner.len() * 8,
            "Length exceeds the number of bits in the inner vector"
        );
        for byte in inner.iter_mut() {
            *byte = 0;
        }
        Self { inner, len }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.len, "Index out of bounds");
        let byte_index = index / 8;
        let bit_index = index % 8;
        if value {
            self.inner[byte_index] |= 1 << bit_index;
        } else {
            self.inner[byte_index] &= !(1 << bit_index);
        }
    }

    pub fn set_all(&mut self, value: bool) {
        let byte_value = if value { 0xFF } else { 0x00 };
        for byte in self.inner.iter_mut() {
            *byte = byte_value;
        }
    }
}

impl<A: alloc::alloc::Allocator> core::ops::Index<usize> for Bitmap<A> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Index out of bounds");
        let byte_index = index / 8;
        let bit_index = index % 8;
        let byte = self.inner[byte_index];
        let bit = (byte >> bit_index) & 1;
        if bit == 1 { &true } else { &false }
    }
}
