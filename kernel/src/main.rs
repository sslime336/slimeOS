#![no_std]
#![no_main]
#![allow(unused)]
#![allow(dead_code)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::{arch::global_asm, panic::PanicInfo};

#[macro_use]
mod macros;

mod console;
mod logging;
mod mm;
mod syscall;
mod timer;
mod trap;
mod uart;
mod config;

extern crate alloc;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn meow() -> ! {
    clear_bss();

    panic!("Goodbye.");
}

fn clear_bss() {
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
    }
    (sbss as usize..ebss as usize).for_each(|b| unsafe { (b as *mut u8).write_volatile(0) });
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(localtion) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            localtion.file(),
            localtion.line(),
            info.message().unwrap()
        );
    } else {
        console::print(format_args!(
            concat!("Paniced: {}", "\n"),
            info.message().unwrap()
        ));
    }

    loop {}
}
