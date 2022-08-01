//! File and filesystem-related syscalls

use crate::task::exit_current_and_run_next;
const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // println!("syswrite buf: {:#X}", buf as usize);
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            if let Ok(str) = core::str::from_utf8(slice) {
                print!("{}", str);
                len as isize
            } else {
                println!("[kernel] write buf error, kernel killed it.");
                exit_current_and_run_next();
                -1
            }
        }
        _ => {
            println!("[kernel] Unsupported fd in sys_write!");
            exit_current_and_run_next();
            -1
        }
    }
}
