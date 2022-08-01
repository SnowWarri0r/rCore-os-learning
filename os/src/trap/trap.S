.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp
    # 切换sp指向内核栈，sscratch指向用户栈
    addi sp, sp, -34*8
    # 将内核栈向下开辟34*8字节的空间
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    # 这里先跳过x2寄存器，因为是用来保存栈顶指针的
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # 这里利用宏循环将x5-x31存入[sp+5*8, sp+31*8]
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # 先把csr寄存器中的sstatus和sepc读入寄存器，然后再保存至内核栈，因为csr寄存器没法直接传递数据至内存
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # 将用户栈顶指针存入内核栈
    mv a0, sp
    # 将当前sp指针传入a0，作为调用规范，给rust函数作为第一个参数，给后续函数调用使用
    call trap_handler

__restore:
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    # 先从栈中读取三个csr寄存器的数据，并将其恢复到csr寄存器
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # 此时sscratch中储存的是用户栈栈顶指针
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    # 这里恢复内核栈至trap之前的状态
    csrrw sp, sscratch, sp
    # 将sp切换回用户栈，sscratch保存内核栈顶指针
    sret