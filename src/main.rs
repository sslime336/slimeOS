#![no_std]
#![no_main]

mod lang_items;
mod sbi;
mod fmt;

use core::arch::global_asm;
global_asm!(include_str!("entry.s"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn bss_entry();
        fn bss_end();
    }

    (bss_entry as usize..bss_end as usize).for_each(|b| unsafe { (b as *mut u8).write_volatile(0) })
}
