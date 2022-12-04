    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw  sp, sscratch, sp
    # TODO
    addi sp, sp, -34*8
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)



    mv a0, sp
    call trap_handler
__restore:
    mv  sp, a0
    ld t0, 32*8(sp)
    # TODO

    csrrw  sp, sscratch, sp # sp = sscratch, sscratch = sp
    sret
