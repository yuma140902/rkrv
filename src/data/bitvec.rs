use crate::allocator::buddy::NoOpAllocator;

extern crate alloc;

pub struct BitVec {
    inner: alloc::vec::Vec<u8, NoOpAllocator>,
    len: usize,
}

impl BitVec {
    pub fn new(inner: alloc::vec::Vec<u8, NoOpAllocator>, len: usize) -> Self {
        assert!(
            len <= inner.len() * 8,
            "Length exceeds the number of bits in the inner vector"
        );
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

impl core::ops::Index<usize> for BitVec {
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
