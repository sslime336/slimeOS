use core::panic::PanicInfo;

use crate::{println, sbi_calls::shutdown};

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
        println!("Paniced: {}", info.message().unwrap());
    }
    shutdown()
}
