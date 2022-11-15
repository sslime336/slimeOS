pub const KERNEL_HEAP_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 20;

pub const USER_STACK_SIZE: usize = 4096 * 2;

pub const PAGE_SIZE: usize = 0x1_000;
pub const PAGE_SIZE_BITS: usize = 0xc;

/// The end address of the kernel
pub const MEMORY_END: usize = 0x80_800_000;

/// Read more about SV39: <https://en.wikipedia.org/wiki/RISC-V#Memory_access>
pub const PHYCIAL_ADDR_WIDTH_SV39: usize = 56;
