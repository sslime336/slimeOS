#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![allow(unused)]

use core::arch::{global_asm, asm};

mod consts;
mod fmt;
mod lang_items;
mod mm;
mod sbi;
mod sbi_calls;
mod sync;
mod trap;

extern crate alloc;

global_asm!(include_str!("entry.s"));

#[no_mangle]
pub fn rust_main() -> ! {
    trap::init();
    clear_bss();
    println!("Hello, slimeOS!");
    
    unsafe {
        asm!("sret");
    }

    panic!("GoodBye!");
}

fn clear_bss() {
    extern "C" {
        fn bss_entry();
        fn bss_end();
    }

    (bss_entry as usize..bss_end as usize).for_each(|b| unsafe { (b as *mut u8).write_volatile(0) })
}
