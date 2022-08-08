mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;
mod memory_set;

use frame_allocator::*;
pub use memory_set::*;
pub use address::*;
pub use page_table::translated_byte_buffer;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}