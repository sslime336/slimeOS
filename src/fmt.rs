use core::fmt::Write;

use crate::sbi::*;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.chars().for_each(|ch| {
            putchar(ch as usize);
        });

        Ok(())
    }
}

pub fn putchar(ch: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}
