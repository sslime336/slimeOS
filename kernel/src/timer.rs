use riscv::register::time;


/// Retures the value of register `mtime`.
pub fn get_time() -> usize {
    time::read()
}
