use super::context::TrapContext;

use riscv::register::{
    scause::{self, Exception::*, Interrupt::*},
    stval,
};

#[no_mangle]
fn trap_handler(ctx: &mut TrapContext) {
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
        scause::Trap::Exception(UserEnvCall) => {}
        scause::Trap::Exception(InstructionPageFault) => {}
        scause::Trap::Exception(LoadPageFault) => {}
        scause::Trap::Exception(StorePageFault) => {}
        scause::Trap::Exception(_) => {
            panic!("unknown exception");
        }
    }
}
