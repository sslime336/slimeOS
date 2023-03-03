use riscv::register::time;

use crate::{
    consts::CLOCK_FREQ,
    sbi::{base::get_impl_id, legacy::set_timer},
};

/// Retures the value of register `mtime`.
pub fn get_time() -> usize {
    time::read()
}
