use crate::sync::SafeCell;
use alloc::vec::Vec;
use lazy_static::lazy_static;

use super::address::{PhyPageNum, PhycialAddress};

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    /// Global frame memory alloctor for vistual page
    pub static ref FRAME_ALLOCTOR: SafeCell<StackFrameAllocator> =
        unsafe { SafeCell::new(FrameAllocatorImpl::new()) };
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhyPageNum>;
    fn dealloc(&mut self, phy_page_num: PhyPageNum);
}

/// Stack-liked frame allocator
pub struct StackFrameAllocator {
    start: usize, // start of the useable memory phycial page number
    end: usize,   // the end of useable phy pages number [start, end)
    recycled: Vec<PhyPageNum>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, start: PhyPageNum, end: PhyPageNum) {
        self.start = start.0;
        self.end = end.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        StackFrameAllocator::default()
    }

    fn alloc(&mut self) -> Option<PhyPageNum> {
        if let Some(addr) = self.recycled.pop() {
            Some(addr.into())
        } else {
            self.start += 1;
            Some((self.start - 1).into())
        }
    }

    fn dealloc(&mut self, phy_page_num: PhyPageNum) {
        if phy_page_num.0 >= self.start || self.recycled.binary_search(&phy_page_num).is_ok() {
            panic!("phycial page {:#x} has not been allocated", phy_page_num.0);
        }

        self.recycled.push(phy_page_num);
    }
}

impl Default for StackFrameAllocator {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
}
