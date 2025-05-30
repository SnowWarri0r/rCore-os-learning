.altmacro
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
    # 这里会让sp指向TrapContext，具体原因见__restore函数
    csrrw sp, sscratch, sp
    # now sp->*TrapContext in user space, sscratch->user stack
    # save other general purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4) thread pointer, application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # t0/t1/t2 = x5/x6/x7
    # we can use t0/t1/t2 freely, because they have been saved in TrapContext
    # 存储当前的sstatus和sepc到TrapContext
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it in TrapContext
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # load kernel_satp into t0
    ld t0, 34*8(sp)
    # load trap_handler into t1
    ld t1, 36*8(sp)
    # move to kernel_sp
    ld sp, 35*8(sp)
    # switch to kernel space
    csrw satp, t0
    # 切换satp后需要刷新TLB
    sfence.vma
    # jump to trap_handler
    # 链接的时候跳板页和trap_handler是贴近的，偏移量比较小
    # 但是跳板页在虚拟地址被映射到了一个极高的地址空间，所以call计算的偏移地址不对
    # 这里必须用jr来跳转，并且我们不需要这个返回地址，因为一切都已经保存到TrapContext中
    # trap_handler和trap_return会处理好一切
    jr t1

__restore:
    # a0: *TrapContext in user space(Constant); a1: user space token
    # switch to user space
    csrw satp, a1
    # 清理TLB
    sfence.vma
    # 放入TrapContext指针到sscratch寄存器
    csrw sscratch, a0
    # sp指向TrapContext
    mv sp, a0
    # now sp points to TrapContext in user space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general purpose registers except x0/sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # back to user stack
    ld sp, 2*8(sp)
    sret