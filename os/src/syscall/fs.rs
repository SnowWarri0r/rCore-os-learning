//! File and filesystem-related syscalls
use crate::batch::{check_invalid};
const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if check_invalid(&buf, &len) {
                println!("[kernel] Invalid memory operation");
                return -1;
            }
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            println!("Unsupported fd in sys_write!");
            -1
        }
    }
}
