mod context;

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    syscall::*,
    task::{
        current_trap_cx, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next,
    },
    timer::set_next_trigger,
};
use core::arch::{asm, global_asm};
use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
    utvec::TrapMode,
};

pub use self::context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe { stvec::write(trap_from_kernel as usize, TrapMode::Direct) }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            )
        }
    }
    trap_return();
}
/// 设置sie的stie使S特权级时钟中断不会被屏蔽
///
/// sstatus的spie没有设置，但是时钟中断能够正常进行，是因为在U特权级运行应用时，S特权级中断可以抢占CPU
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}
