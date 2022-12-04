use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Interrupt},
    sepc, sscratch, stval, stvec,
};

use crate::println;

use super::context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    // save all registers
    extern "C" {
        fn __alltraps();
    }

    sscratch::write(0); // trap from kernel
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("trap init finished");
}

#[no_mangle]
fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        scause::Trap::Interrupt(USER_SOFT) => {}
        scause::Trap::Interrupt(SUPERVISOR_SOFT) => {}
        scause::Trap::Interrupt(USER_TIMER) => {}
        scause::Trap::Interrupt(SUPERVISOR_TIMER) => {}
        scause::Trap::Interrupt(USER_EXTERNAL) => {}
        scause::Trap::Interrupt(SUPERVISOR_EXTERNAL) => {}
        scause::Trap::Interrupt(_) => {} // as UNKNOWN
        scause::Trap::Exception(INSTRUCTION_MISALIGNED) => {}
        scause::Trap::Exception(INSTRUCTION_FAULT) => {}
        scause::Trap::Exception(ILLEGAL_INSTRUCTION) => {}
        scause::Trap::Exception(BREAKPOINT) => {}
        scause::Trap::Exception(LOAD_FAULT) => {}
        scause::Trap::Exception(STORE_FAULT) => {}
        scause::Trap::Exception(USER_ENV_CALL) => {}
        scause::Trap::Exception(INSTRUCTION_PAGE_FAULT) => {}
        scause::Trap::Exception(LOAD_PAGE_FAULT) => {}
        scause::Trap::Exception(STORE_PAGE_FAULT) => {}
        scause::Trap::Exception(_) => {} // as UNKNOWN,
    }
    ctx
}

// Exceptions
const INSTRUCTION_MISALIGNED: Exception = riscv::register::scause::Exception::InstructionMisaligned;
const INSTRUCTION_FAULT: Exception = riscv::register::scause::Exception::InstructionFault;
const ILLEGAL_INSTRUCTION: Exception = riscv::register::scause::Exception::IllegalInstruction;
const BREAKPOINT: Exception = riscv::register::scause::Exception::Breakpoint;
const LOAD_FAULT: Exception = riscv::register::scause::Exception::LoadFault;
const STORE_MISALIGNED: Exception = riscv::register::scause::Exception::StoreMisaligned;
const STORE_FAULT: Exception = riscv::register::scause::Exception::StoreFault;
const USER_ENV_CALL: Exception = riscv::register::scause::Exception::UserEnvCall;
const INSTRUCTION_PAGE_FAULT: Exception = riscv::register::scause::Exception::InstructionPageFault;
const LOAD_PAGE_FAULT: Exception = riscv::register::scause::Exception::LoadPageFault;
const STORE_PAGE_FAULT: Exception = riscv::register::scause::Exception::StorePageFault;
// const UNKNOWN: Exception = riscv::register::scause::Exception::Unknown;

// Interrupts
const USER_SOFT: Interrupt = riscv::register::scause::Interrupt::UserSoft;
const SUPERVISOR_SOFT: Interrupt = riscv::register::scause::Interrupt::SupervisorSoft;
const USER_TIMER: Interrupt = riscv::register::scause::Interrupt::UserTimer;
const SUPERVISOR_TIMER: Interrupt = riscv::register::scause::Interrupt::UserExternal;
const USER_EXTERNAL: Interrupt = riscv::register::scause::Interrupt::UserExternal;
const SUPERVISOR_EXTERNAL: Interrupt = riscv::register::scause::Interrupt::SupervisorExternal;
// const UNKNOWN: Interrupt = riscv::register::scause::Interrupt::Unknown;
