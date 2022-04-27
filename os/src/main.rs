#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod sync;
pub mod batch;
mod lang_items;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));
// 防止编译器对函数符号混淆
#[no_mangle]
pub fn rust_main() {
    extern "C" {
        fn stext();
        fn etext();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn sstack();
        fn estack();
        fn srodata();
        fn erodata();
    }
    clear_bss();
    info!(".text, [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    debug!("boot_stack [{:#x}, {:#x})", sstack as usize, estack as usize);
    info!("Hello, World!");
    warn!("Hello, World!");
    error!("Hello, World!");
    trace!("Hello, World!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
    for a in sbss as usize..ebss as usize {
        unsafe { (a as *mut u8).write_volatile(0) }
    }
}
