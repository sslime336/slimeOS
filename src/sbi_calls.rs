use crate::sbi::*;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("shutdown");
}

pub fn console_putchar(ch: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}

pub fn console_getchar(ch: usize) {
    sbi_call(SBI_CONSOLE_GETCHAR, ch, 0, 0);
}

pub fn clear_timer(t: usize) {
    sbi_call(SBI_SET_TIMER, t, 0, 0);
}
