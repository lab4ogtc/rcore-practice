mod context;
mod switch;
mod task;

use alloc::vec::Vec;
use crate::config::{CLOCK_FREQ, MAX_APP_LIFETIME_CLOCK, MSEC_PER_SEC};
use crate::loader::{get_num_app, get_app_data, get_app_name};
use crate::timer::get_time;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus, BIG_STRIDE};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        info!("init TASK_MANAGER");
        let num_app = get_num_app();
        info!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(
                get_app_data(i),
                i,
                get_app_name(i),
            ));
        }
        TaskManager {
            num_app,
            inner: unsafe {UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })},
        }
    };
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_elapse_time +=
            get_time() - inner.tasks[current].task_last_switch_time;
        if inner.tasks[current].task_elapse_time > MAX_APP_LIFETIME_CLOCK {
            inner.tasks[current].task_status = TaskStatus::Exited;
            warn!(
                "[kernel] Force stop the long lifetime app({}) which maybe dead loop",
                inner.tasks[current].task_name
            );
            info!(
                "[kernel] {} executed for {}ms",
                inner.tasks[current].task_name,
                inner.tasks[current].task_elapse_time / (CLOCK_FREQ / MSEC_PER_SEC)
            );
        } else {
            inner.tasks[current].task_status = TaskStatus::Ready;
        }
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_elapse_time +=
            get_time() - inner.tasks[current].task_last_switch_time;
        inner.tasks[current].task_status = TaskStatus::Exited;
        info!(
            "[kernel] {} executed for {}ms",
            inner.tasks[current].task_name,
            inner.tasks[current].task_elapse_time / (CLOCK_FREQ / MSEC_PER_SEC)
        );
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .filter(|id| {
                let current_time = get_time();
                inner.tasks[*id].task_status == TaskStatus::Ready
                    && inner.tasks[*id].task_awake_time < current_time
            })
            .reduce(|left, right| {
                if inner.tasks[left].task_stride > inner.tasks[right].task_stride {
                    right
                } else {
                    left
                }
            })
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.tasks[next].task_stride.value += BIG_STRIDE / inner.tasks[next].task_priority;
            inner.tasks[next].task_last_switch_time = get_time();
            inner.current_task = next;

            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            trace!(
                "[kernel] run next task {} with stride={}",
                inner.tasks[next].task_name,
                inner.tasks[next].task_stride.value,
            );
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn run_first_task(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.tasks[0].task_status = TaskStatus::Running;
        inner.tasks[0].task_last_switch_time = get_time();
        let next_task_cx_ptr = &inner.tasks[0].task_cx as *const TaskContext;
        let next_task_name = inner.tasks[0].task_name;
        core::mem::drop(inner);
        let mut _unused = TaskContext::zero_init();
        debug!("[kernel] run first task {}", next_task_name);
        unsafe {
            __switch(&mut _unused as *mut _, next_task_cx_ptr);
        }
    }

    fn get_current_app(&self) -> usize {
        self.inner.exclusive_access().current_task
    }

    fn update_current_task_priority(&self, prio: u16) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_priority = prio;
    }

    fn current_sleep_for_ticks(&self, ticks: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_awake_time = get_time() + ticks;
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }

    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn set_current_task_priority(prio: u16) {
    TASK_MANAGER.update_current_task_priority(prio);
}

pub fn current_sleep_for_ticks(ticks: usize) {
    TASK_MANAGER.current_sleep_for_ticks(ticks);
    suspend_current_and_run_next();
}

pub use context::TaskContext;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
