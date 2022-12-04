use core::fmt::Debug;

use riscv::register::{scause::Scause, sstatus::Sstatus};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub scause: Scause, // as Scause's bits field
    pub sepc: usize,
    pub stval: usize,
}
