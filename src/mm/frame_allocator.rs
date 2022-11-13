use alloc::vec::Vec;

use super::address::PhysPageNum;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, phy_page_num: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize, // free memory phycial page number
    end: usize,     // end phy page num
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        StackFrameAllocator::default()
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(addr) = self.recycled.pop() {
            Some(addr.into())
        } else {
            None
        }
    }

    fn dealloc(&mut self, phy_page_num: PhysPageNum) {
        todo!()
    }
}

impl Default for StackFrameAllocator {
    fn default() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
}
