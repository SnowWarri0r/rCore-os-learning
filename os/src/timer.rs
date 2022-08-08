use riscv::register::time;
use crate::{config::CLOCK_FREQ, sbi::set_timer};
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