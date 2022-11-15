use alloc::vec::Vec;

use super::address::PhyPageNum;

pub struct PageTable {
    root_phy_page_num: PhyPageNum,
    frames: Vec<>
}
