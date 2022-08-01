#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[no_mangle]
fn main() -> i32 {
    println!("I'm here");
    let current_timer = get_time();
    let wait_for = current_timer + 30000;
    while get_time() < wait_for {
        println!("blablabla");
    }
    println!("Test sleep OK!");
    0
}
