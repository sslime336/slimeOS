/// Reference:
/// <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#5-legacy-extensions-eids-0x00---0x0f>
use super::SbiRet;
use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }

    ret
}

/// Returns 0 upon success or an implementation specific negative error code.
/// 
/// Programs the clock for next event after stime_value time.
/// 
/// This function also clears the pending timer interrupt bit.
/// 
/// See more:
/// <http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter3/4time-sharing-system.html#id6>
pub fn set_timer(stime_value: usize) -> isize {
    sbi_call(SBI_SET_TIMER, stime_value, 0, 0) as isize
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar(ch: usize) {
    sbi_call(SBI_CONSOLE_GETCHAR, ch, 0, 0);
}

pub fn clear_ipi() {
    sbi_call(SBI_CLEAR_IPI, 0, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("shutdown");
}

pub fn clear_timer(t: usize) {
    sbi_call(SBI_SET_TIMER, t, 0, 0);
}
