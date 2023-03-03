.altmacro    # for using .rept 
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp

    .set n, 1
    .rept 31
        SAVE_GP %n
        .set n, n+1
    .endr

    csrr t0, sstatus
    sd t0, 32*8(sp)
    csrr t0, sepc
    sd t0, 33*8(sp)
    csrr t0, sscratch
    sd t0, 2*8(sp)

    ld t0, 34*8(sp)
    ld t1, 36*8(sp)
    ld sp, 35*8(sp)
    csrw satp, t0
    sfence.vma

    jr t1

__restore:
    # a0: *TrapContext in user space(Constant); a1: user space token

    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp, a0

    ld t0, 32*8(sp)
    csrw sstatus, t0
    ld t0, 33*8(sp)
    csrw sepc, t0

    ld x1, 1*8(sp)
    
    .set n, 3
    .rept 29
        LOAD_GP %n
        .set n, n+1
    .endr

    ld sp, 2*8(sp)
    sret