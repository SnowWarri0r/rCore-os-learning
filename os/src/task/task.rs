/// 线程块实现
use core::cell::RefMut;

use alloc::{string::String, sync::Arc, sync::Weak, vec, vec::Vec};

use crate::{
    fs::{File, Stdin, Stdout},
    mm::{translated_refmut, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE},
    sync::UPSafeCell,
    task::SignalFlags,
    trap::{trap_handler, TrapContext},
};

use super::{
    id::{kstack_alloc, pid_alloc, KernelStack, PidHandle, TaskUserRes},
    process::ProcessControlBlock,
    TaskContext,
};

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Blocked,
}

/// 线程的基本单位，线程控制块
pub struct TaskControlBlock {
    // immutable
    pub process: Weak<ProcessControlBlock>,
    pub kstack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}
pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    pub trap_cx_ppn: PhysPageNum,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub exit_code: Option<i32>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_status(&self) -> TaskStatus {
        self.task_status
    }
}
impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.memory_set.token()
    }
    /// 分配内核栈，分配TrapContext在用户空间中，并且把TrapContext的返回地址设置为跳板，寄存器清空，准备第一次调度执行
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    res: Some(res),
                    trap_cx_ppn,
                    task_cx: TaskContext::goto_trap_return(kstack_top),
                    task_status: TaskStatus::Ready,
                    exit_code: None,
                })
            },
        }
    }
}
