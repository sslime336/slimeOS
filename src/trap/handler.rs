use core::arch::global_asm;

use super::context::TrapContext;
use crate::{println, trap::handler};
use riscv::register::{
    scause::{self, Exception::*, Interrupt::*},
    sepc, sscratch, stval, stvec,
    utvec::TrapMode,
};

#[no_mangle]
fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    // Give the reason of current trap.
    let scause = scause::read();

    // Scause is set according to the cause, stval s set to the address of the
    // error or other exception-specific message word.
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
            panic!("unknown interrupt");
        }

        // Exceptions
        scause::Trap::Exception(InstructionMisaligned) => {}
        scause::Trap::Exception(InstructionFault) => {}
        scause::Trap::Exception(IllegalInstruction) => {}
        scause::Trap::Exception(Breakpoint) => {}
        scause::Trap::Exception(LoadFault) => {}
        scause::Trap::Exception(StoreFault) => {}
        scause::Trap::Exception(UserEnvCall) => {
            ctx.sepc += 4;
            let result = sys_call(
                ctx.x[17],
                [
                    ctx.x[10], ctx.x[11], ctx.x[12], ctx.x[13], ctx.x[14], ctx.x[15],
                ],
            );
            ctx = current_trap_cx();
            ctx.x[10] = result as usize;
        }
        scause::Trap::Exception(InstructionPageFault) => {}
        scause::Trap::Exception(LoadPageFault) => {}
        scause::Trap::Exception(StorePageFault) => {}
        scause::Trap::Exception(_) => {
            panic!("unknown exception");
        }
    }

    trap_frame
}
