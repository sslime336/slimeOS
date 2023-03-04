/// Reference:
/// <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#4-base-extension-eid-0x10>
use super::{sbi_call, SbiRet, EID_BASE};
use core::arch::asm;

/// Returns the current SBI specification version.
///
/// This function will never failed.
///
/// The minor number of the SBI specification is encoded in the low 24 bits,
/// with the major number encoded in the next 7 bits.
pub fn get_spec_version() -> usize {
    let ret = sbi_call(EID_BASE, 0, 0, 0);
    let version = ret.value;

    version
}

/// Returns the current SBI implementation ID, which is different
/// for every SBI implementation.
pub fn get_impl_id() -> usize {
    let ret = sbi_call(EID_BASE, 1, 0, 0);
    let impl_id = ret.value;

    impl_id
}
