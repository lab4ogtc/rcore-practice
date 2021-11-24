pub mod address;
mod frame_allocator;
mod heap_allocator;
mod page_table;
pub mod memory_set;

pub use page_table::translated_byte_buffer;
use crate::mm::memory_set::KERNEL_SPACE;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}