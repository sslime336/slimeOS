#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]

use core::arch::{asm, global_asm};

use crate::sbi::{
    base::{get_impl_id, get_spec_version},
    legacy::{set_timer, shutdown},
};

mod consts;
mod fmt;
mod lang_items;
mod mm;
mod sbi;
mod sync;
mod timer;
mod trap;

extern crate alloc;

global_asm!(include_str!("entry.s"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    println!("impl id: {}", get_impl_id());
    println!("sbi version: {}", get_spec_version());

    println!("here start sleeping for 1 second");
    set_timer(3_000_000_0);
    println!("one second has passed");

    shutdown()
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|b| unsafe { (b as *mut u8).write_volatile(0) })
}
