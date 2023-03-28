use core::sync::atomic::AtomicBool;

pub static mut BOOTED: AtomicBool = core::sync::atomic::AtomicBool::new(false);

macro_rules! synchronize_hart {
    () => {{
        unsafe {
            $crate::macros::on_boot::BOOTED.store(true, core::sync::atomic::Ordering::Relaxed);
            core::arch::asm!("fence");
        }
    }};
}

macro_rules! wait_for_booting {
    () => {{
        unsafe {
            while !$crate::macros::on_boot::BOOTED.load(core::sync::atomic::Ordering::Acquire) {}
        }
    }};
}
