pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 4;
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 256;
pub const USER_HEAP_SIZE: usize = PAGE_SIZE * 48;

pub const MEMORY_END: usize = 0x807E0000;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub const MMAP_BASE: usize = 0x60000000;

pub const CLOCK_FREQ: usize = 12500000;

pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x1000)];
