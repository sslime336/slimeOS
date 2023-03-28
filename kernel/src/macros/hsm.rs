macro_rules! get_hartid {
    () => { {
        let hartid: usize;
        unsafe {core::arch::asm!("mv {}, tp", out(reg) hartid)}

        hartid
    }};
}
