use core::panic::PanicInfo;

use crate::{fmt, println, sbi::legacy::shutdown};

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
        fmt::print(format_args!(
            concat!("Paniced: {}", "\n"),
            info.message().unwrap()
        ));
    }
    shutdown()
}
