#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhycialAddress(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAdderss(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysPageNum(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<usize> for PhysPageNum {
    fn from(phy_page_num: usize) -> Self {
        PhysPageNum(phy_page_num)
    }
}
impl From<usize> for VirtPageNum {
    fn from(virt_page_num: usize) -> Self {
        VirtPageNum(virt_page_num)
    }
}

const PHYCIAL_ADDR_WIDTH_SV39: usize = 56;
// const PHYCIAL_PAGE_NUM_WIDTH_SV39:usize = PHYCIAL_ADDR_WIDTH_SV39 - Pag
