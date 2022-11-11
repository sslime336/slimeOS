#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![allow(unused)]

mod fmt;
mod lang_items;
mod mm;
mod sbi;
mod trap;
mod consts;

extern crate alloc;
use core::arch::global_asm;

global_asm!(include_str!("entry.s"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, slimeOS!");
    panic!("GoodBye!");
}

fn clear_bss() {
    extern "C" {
        fn bss_entry();
        fn bss_end();
    }

    (bss_entry as usize..bss_end as usize).for_each(|b| unsafe { (b as *mut u8).write_volatile(0) })
}
