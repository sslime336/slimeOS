//! Find more about riscv-sbi below:
//! <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#introduction>

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
