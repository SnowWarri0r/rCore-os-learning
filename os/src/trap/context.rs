use riscv::register::sstatus::Sstatus;

use riscv::register::sstatus::{self, SPP};
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// 构建应用程序执行的初始Trap上下文
    // 设置特权级为用户态
    // 寄存器全部初始化为0
    // 指定程序入口为entry
    // 指定用户栈栈顶指针为sp
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
