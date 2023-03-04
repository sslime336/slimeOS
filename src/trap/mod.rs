use core::arch::global_asm;

use riscv::register::{sscratch, stvec};

use crate::println;

mod context;
pub mod handler;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    sscratch::write(0); // trap from kernel
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("trap init finished");
}
