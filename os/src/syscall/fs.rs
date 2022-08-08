//! File and filesystem-related syscalls

use crate::{mm::translated_byte_buffer, task::{exit_current_and_run_next, current_user_token}};
const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            println!("[kernel] Unsupported fd in sys_write!");
            exit_current_and_run_next();
            -1
        }
    }
}
