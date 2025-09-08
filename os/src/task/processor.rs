use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::{sync::UPSafeCell, trap::TrapContext};

use super::{
    context::TaskContext,
    manager::fetch_task,
    switch::__switch,
    task::{TaskControlBlock, TaskStatus}, process::ProcessControlBlock,
};

pub struct Processor {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx: TaskContext,
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
}
/// 取出当前处理器正在执行的线程
pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}
/// 当前线程控制块
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}
/// 当前进程控制块
pub fn current_process() -> Arc<ProcessControlBlock> {
    current_task().unwrap().process.upgrade().unwrap()
}
/// 当前进程地址空间satp
pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.get_user_token()
}
///当前线程Trap上下文
pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}
/// 当前trap上下文在用户空间的虚拟地址
pub fn current_trap_cx_user_va() -> usize {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .trap_cx_user_va()
}
/// 当前线程内核栈栈顶
pub fn current_kstack_top() -> usize {
    current_task().unwrap().kstack.get_top()
}
/// CPU 的调度主循环
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // stop exclusively accessing coming task TCB manually
            drop(task_inner);
            processor.current = Some(task);
            // stop exclusively accessing processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("no tasks available in run_tasks");
        }
    }
}

// 保存当前进程上下文，提取下一个要运行的进程的上下文到寄存器，并恢复堆栈指针。
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
