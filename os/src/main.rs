#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
#[macro_use]
mod log;
mod config;
mod lang_items;
mod loader;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;
mod mm;
mod sync;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    mm::memory_set::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
}
