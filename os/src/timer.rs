use core::cmp::Ordering;

use crate::sync::UPSafeCell;
use crate::task::wakeup_task;
use crate::{config::CLOCK_FREQ, sbi::set_timer, task::TaskControlBlock};
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use lazy_static::*;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
const MICRO_PER_SEC: usize = 1_000_000;

pub fn get_time() -> usize {
    time::read()
}
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
// 设置下一次时钟中断触发，借此建立时间片抢占机制
pub fn set_next_trigger() {
    // CLOCK_FREQ 是时钟频率，单位为Hz，代表一秒内计数器的增量
    // 通过获取mtime的值加上增量设置mtimecmp，这样10ms后就会触发S特权级时钟中断
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub struct TimerCondVar {
    pub expire_ms: usize,
    pub task: Arc<TaskControlBlock>,
}

impl PartialEq for TimerCondVar {
    fn eq(&self, other: &Self) -> bool {
        self.expire_ms == other.expire_ms
    }
}
impl Eq for TimerCondVar {}
impl PartialOrd for TimerCondVar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = -(self.expire_ms as isize);
        let b = -(other.expire_ms as isize);
        Some(a.cmp(&b))
    }
}

impl Ord for TimerCondVar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

lazy_static! {
    static ref TIMERS: UPSafeCell<BinaryHeap<TimerCondVar>> =
        unsafe { UPSafeCell::new(BinaryHeap::<TimerCondVar>::new()) };
}

pub fn add_timer(expire_ms: usize, task: Arc<TaskControlBlock>) {
    let mut timers = TIMERS.exclusive_access();
    timers.push(TimerCondVar { expire_ms, task });
}

pub fn remove_timer(task: Arc<TaskControlBlock>) {
    let mut timers = TIMERS.exclusive_access();
    let mut temp = BinaryHeap::<TimerCondVar>::new();
    for condvar in timers.drain() {
        if Arc::as_ptr(&task) != Arc::as_ptr(&condvar.task) {
            temp.push(condvar);
        }
    }
    timers.clear();
    timers.append(&mut temp);
}

pub fn check_timer() {
    let current_ms = get_time_ms();
    let mut timers = TIMERS.exclusive_access();
    while let Some(timer) = timers.peek() {
        if timer.expire_ms <= current_ms {
            wakeup_task(Arc::clone(&timer.task));
            timers.pop();
        } else {
            break;
        }
    }
}
