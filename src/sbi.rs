#![allow(unused)]

use core::arch::asm;

pub(crate) const SBI_SET_TIMER: usize = 0;
pub(crate) const SBI_CONSOLE_PUTCHAR: usize = 1;
pub(crate) const SBI_CONSOLE_GETCHAR: usize = 2;
pub(crate) const SBI_CLEAR_IPI: usize = 3;
pub(crate) const SBI_SEND_IPI: usize = 4;
pub(crate) const SBI_REMOTE_FENCE_I: usize = 5;
pub(crate) const SBI_REMOTE_SFENCE_VMA: usize = 6;
pub(crate) const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
pub(crate) const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
pub fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg1,
            in("x17") which,
        );
    }
    ret
}
