#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod fmt;
mod lang_items;
mod sbi;

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
