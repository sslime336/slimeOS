use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception::*, Interrupt::*},
    sepc, sscratch, stval, stvec,
};

use crate::println;

use super::context::TrapContext;

global_asm!(include_str!("trap.S"));

#[no_mangle]
fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // Interrupt
        scause::Trap::Interrupt(UserSoft) => {}
        scause::Trap::Interrupt(SupervisorSoft) => {}
        scause::Trap::Interrupt(UserTimer) => {}
        scause::Trap::Interrupt(SupervisorTimer) => {}
        scause::Trap::Interrupt(UserExternal) => {}
        scause::Trap::Interrupt(SupervisorExternal) => {}
        scause::Trap::Interrupt(_) => {} // as UNKNOWN

        // Exception
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
        scause::Trap::Exception(_) => {} // as UNKNOWN,
    }

    println!("{:?}", ctx.sstatus);
    ctx;
    panic!("fin")
}
