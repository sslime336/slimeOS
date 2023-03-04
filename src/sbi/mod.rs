//! Find more about riscv-sbi below:
//! <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#introduction>

use core::arch::asm;

pub(crate) mod base;
pub(crate) mod legacy;

struct SbiRet {
    error: SbiError,
    value: usize,
}

enum SbiError {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
}

impl From<isize> for SbiError {
    fn from(error: isize) -> Self {
        match error {
            0 => Self::Success,
            -1 => Self::Failed,
            -2 => Self::NotSupported,
            -3 => Self::InvalidParam,
            -4 => Self::Denied,
            -5 => Self::InvalidAddress,
            -6 => Self::AlreadyAvailable,
            -7 => Self::AlreadyStarted,
            -8 => Self::AlreadyStopped,
            _ => unreachable!(),
        }
    }
}

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

// Reference:
// <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#risc-v-supervisor-binary-interface-specification>

const EID_BASE: usize = 0x10;
const EID_TIME: usize = 0x54494D45;
const EID_IPI: usize = 0x735049;
const EID_RFNC: usize = 0x52464E43;
const EID_HSM: usize = 0x48534D;
const EID_SRST: usize = 0x53525354;
const EID_PMU: usize = 0x504D55;
const EID_DBCN: usize = 0x4442434E;
