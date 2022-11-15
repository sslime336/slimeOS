use crate::consts::PAGE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhycialAddress(pub usize);

impl PhycialAddress {
    pub fn floor(&self) -> PhycialAddress {
        PhycialAddress(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhycialAddress {
        PhycialAddress((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAdderss(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhyPageNum(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhycialAddress {
    fn from(addr: usize) -> Self {
        PhycialAddress(addr)
    }
}

impl From<usize> for VirtualAdderss {
    fn from(addr: usize) -> Self {
        VirtualAdderss(addr)
    }
}

impl From<usize> for PhyPageNum {
    fn from(phy_page_num: usize) -> Self {
        PhyPageNum(phy_page_num)
    }
}
impl From<usize> for VirtPageNum {
    fn from(virt_page_num: usize) -> Self {
        VirtPageNum(virt_page_num)
    }
}
