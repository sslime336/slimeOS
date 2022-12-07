.equ PLF_SIZE, 8 # platform related, here is x64, 8 bytes

.macro STORE_REG n
    sd x\n, \n*8(sp) 
.endm
.macro LOAD_REG n
    ld x\n, \n
.endm
.macro STORE d, n
    sd \d, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw  sp, sscratch, sp
    # judge: trap from U-mode or kernel
    bnez sp, u_trap # current sp is not zero, the trap is from U-mode

    # trap from kernel, recover the sp on kernel stack
    csrr sp, sscratch
u_trap:
    addi sp, sp, -36*PLF_SIZE
    STORE_REG 1
    STORE_REG 3
    STORE_REG 4
    STORE_REG 5
    STORE_REG 6
    STORE_REG 7
    STORE_REG 8
    STORE_REG 9
    STORE_REG 10
    STORE_REG 11
    STORE_REG 12
    STORE_REG 13
    STORE_REG 14
    STORE_REG 15
    STORE_REG 16
    STORE_REG 17
    STORE_REG 18
    STORE_REG 19
    STORE_REG 20
    STORE_REG 21
    STORE_REG 22
    STORE_REG 23
    STORE_REG 24
    STORE_REG 25
    STORE_REG 26
    STORE_REG 27
    STORE_REG 28
    STORE_REG 29
    STORE_REG 30
    STORE_REG 31

    csrrw s0,sscratch,x0
    csrr s1, sstatus
    csrr s2, sepc
    csrr s3, stval
    csrr s4, scause
    STORE s0, 2
    STORE s1, 32
    STORE s2, 33
    STORE s3, 34
    STORE s4, 35
    mv a0, sp
    jal trap_handler # call trap_handler
__restore:
    mv  sp, a0
    ld t0, 32*8(sp)
    # TODO

    csrrw  sp, sscratch, sp # sp = sscratch, sscratch = sp
    sret
