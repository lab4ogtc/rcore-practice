use crate::config::{CLOCK_FREQ, MSEC_PER_SEC};
use crate::task::{
    current_sleep_for_ticks, exit_current_and_run_next, set_current_task_priority,
    suspend_current_and_run_next,
};
use crate::timer::get_time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_set_priority(prio: isize) -> isize {
    if prio < 2 {
        return -1;
    }
    set_current_task_priority(if prio > (u16::MAX as isize) { u16::MAX } else { prio as u16 });
    prio
}

pub fn sys_sleep(milliseconds: usize) -> isize {
    let ticks = milliseconds * CLOCK_FREQ / MSEC_PER_SEC;
    current_sleep_for_ticks(ticks);
    0
}
