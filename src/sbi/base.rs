/// Reference:
/// <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#4-base-extension-eid-0x10>
use super::SbiRet;
use core::arch::asm;

pub(crate) const EID: usize = 0x10;

#[inline(always)]
fn sbi_call(extension_id: usize, function_id: usize, arg0: usize, arg1: usize) -> SbiRet {
    let (error, value): (isize, usize);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0, in("a1") arg1,
                in("a6") function_id, in("a7") extension_id,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };

    SbiRet {
        error: error.into(),
        value,
    }
}

/// Returns the current SBI specification version.
///
/// This function will never failed.
///
/// The minor number of the SBI specification is encoded in the low 24 bits,
/// with the major number encoded in the next 7 bits.
pub fn get_spec_version() -> usize {
    let ret = sbi_call(0x10, 0, 0, 0);
    let version = ret.value;

    version
}

/// Returns the current SBI implementation ID, which is different
/// for every SBI implementation.
pub fn get_impl_id() -> usize {
    let ret = sbi_call(0x10, 1, 0, 0);
    let impl_id = ret.value;

    impl_id
}
