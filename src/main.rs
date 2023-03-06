#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(unused)]

use core::arch::{asm, global_asm};

use crate::{
    mm::heap_allocator::{heap_test, init_heap},
    sbi::{
        base::{get_impl_id, get_spec_version},
        legacy::{set_timer, shutdown},
    },
};

mod batch;
mod consts;
mod fmt;
mod lang_items;
mod mm;
mod sbi;
mod sync;
mod syscall;
mod timer;
mod trap;

extern crate alloc;

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    println!("impl id: {}", get_impl_id());
    println!("sbi version: {}", get_spec_version());

    init_heap();
    heap_test();

    panic!("Goodbye.");
}

fn clear_bss() {
    println!("\n\n---------------slimeOS---------------");
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    };
    (sbss as usize..ebss as usize).for_each(|b| unsafe { (b as *mut u8).write_volatile(0) });
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}
