mod inode;
mod stdio;

use crate::mm::UserBuffer;
pub use inode::{open_file, OpenFlags, list_apps};
pub use stdio::{Stdin, Stdout};

/// os 层面抽象的 file 概念
pub trait File : Send + Sync {
     /// If readable
     fn readable(&self) -> bool;
     /// If writable
     fn writable(&self) -> bool;
    /// 读取数据到 `UserBuffer`
    fn read(&self, buf: UserBuffer) -> usize;
    /// 写入数据到 `UserBuffer`
    fn write(&self, buf: UserBuffer) -> usize;
}