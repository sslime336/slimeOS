use core::arch::global_asm;

use super::context::TrapFrame;
use crate::{println, trap::trap_handler};
use riscv::register::{
    scause::{self, Exception::*, Interrupt::*},
    sepc, sscratch, stval, stvec,
};

global_asm!(include_str!("trap.S"));

#[no_mangle]
fn trap_handler(trap_frame: &mut TrapFrame) -> &mut TrapFrame {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // Interrupts
        scause::Trap::Interrupt(UserSoft) => {}
        scause::Trap::Interrupt(SupervisorSoft) => {}
        scause::Trap::Interrupt(UserTimer) => {}
        scause::Trap::Interrupt(SupervisorTimer) => {}
        scause::Trap::Interrupt(UserExternal) => {}
        scause::Trap::Interrupt(SupervisorExternal) => {}
        scause::Trap::Interrupt(_) => {
            panic!("Unknown interrupt!");
        }

        // Exceptions
        scause::Trap::Exception(InstructionMisaligned) => {}
        scause::Trap::Exception(InstructionFault) => {}
        scause::Trap::Exception(IllegalInstruction) => {}
        scause::Trap::Exception(Breakpoint) => {}
        scause::Trap::Exception(LoadFault) => {}
        scause::Trap::Exception(StoreFault) => {}
        scause::Trap::Exception(UserEnvCall) => {}
        scause::Trap::Exception(InstructionPageFault) => {}
        scause::Trap::Exception(LoadPageFault) => {}
        scause::Trap::Exception(StorePageFault) => {}
        scause::Trap::Exception(_) => {
            panic!("Unknown exception!");
        }
    }

    trap_frame
}
