.equ PLF_SIZE, 8 # platform related, here is x64, 8 bytes

.macro STORE_REG n
    sd x\n, \n*8(sp) 
.endm
.macro STORE d, n
    sd \d, \n*8(sp)
.endm
.macro LOAD_REG n
    ld x\n, \n*8(sp)
.endm
.macro LOAD d, n
    ld \d, \n*8(sp)
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
    csrr s0, sscratch # sscratch means sp on kernel stack before `addi`
    csrr s1, sstatus
    csrr s2, sepc
    csrr s3, stval
    csrr s4, scause
    STORE s0, 2  # sscratch
    STORE s1, 33 # sstatus
    STORE s2, 34 # spec
    STORE s3, 35 # stval
    STORE s4, 36 # scause
    mv ra, sp
    call trap_handler # call trap_handler
__restore:
    LOAD s1, 33 # sstatus
    LOAD s2, 34 # spec
    addi s0, s1, 1 << 8
    bnez s0, _back_kernel
_back_u:
    csrw sscratch, s0
    csrw sstatus, s1
    csrw sepc, s2
    csrw stval, s3
    csrw scause, s4

    LOAD_REG 1
    LOAD_REG 3
    LOAD_REG 4
    LOAD_REG 5
    LOAD_REG 6
    LOAD_REG 7
    LOAD_REG 8
    LOAD_REG 9
    LOAD_REG 10
    LOAD_REG 11
    LOAD_REG 12
    LOAD_REG 13
    LOAD_REG 14
    LOAD_REG 15
    LOAD_REG 16
    LOAD_REG 17
    LOAD_REG 18
    LOAD_REG 19
    LOAD_REG 20
    LOAD_REG 21
    LOAD_REG 22
    LOAD_REG 23
    LOAD_REG 24
    LOAD_REG 25
    LOAD_REG 26
    LOAD_REG 27
    LOAD_REG 28
    LOAD_REG 29
    LOAD_REG 30
    LOAD_REG 31

    addi sp, sp, 36*PLF_SIZE
    csrrw sp, sscratch, sp
    sret