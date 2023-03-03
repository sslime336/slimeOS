use riscv::register::{sscratch, stvec};

use crate::println;

mod context;
pub mod handler;

pub fn init() {
    // save all registers
    extern "C" {
        fn __alltraps();
    }

    sscratch::write(0); // trap from kernel
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("trap init finished");
}
