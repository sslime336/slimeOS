use crate::config::PAGE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhycialAddress(pub usize);

impl PhycialAddress {
    pub fn floor(&self) -> PhyPageNum {
        PhyPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhyPageNum {
        PhyPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAdderss(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhyPageNum(pub usize);

impl PhyPageNum {
    // pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
    //     let pa: PhysAddr = (*self).into();
    //     unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512) }
    // }
    // /// 返回一个字节数组的可变引用，可以以字节为粒度对物理页帧上的数据进行访问
    // pub fn get_bytes_array(&self) -> &'static mut [u8] {
    //     let pa: PhysAddr = (*self).into();
    //     unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    // }
    // /// 获取一个恰好放在一个物理页帧开头的类型为 T 的数据的可变引用
    // pub fn get_mut<T>(&self) -> &'static mut T {
    //     let pa: PhysAddr = (*self).into();
    //     unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    // }
}

impl From<usize> for PhyPageNum {
    fn from(phy_page_num: usize) -> Self {
        PhyPageNum(phy_page_num)
    }
}

impl From<PhyPageNum> for usize {
    fn from(num: PhyPageNum) -> Self {
        num.0
    }
}

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

impl From<usize> for VirtPageNum {
    fn from(virt_page_num: usize) -> Self {
        VirtPageNum(virt_page_num)
    }
}

impl From<PhycialAddress> for usize {
    fn from(addr: PhycialAddress) -> Self {
        addr.0
    }
}

impl From<VirtualAdderss> for usize {
    fn from(addr: VirtualAdderss) -> Self {
        addr.0
    }
}

impl From<VirtPageNum> for usize {
    fn from(num: VirtPageNum) -> Self {
        num.0
    }
}
