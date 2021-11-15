mod context;
mod switch;
mod task;

use crate::config::{CLOCK_FREQ, MAX_APP_LIFETIME_CLOCK, MAX_APP_NUM, MSEC_PER_SEC};
use crate::loader::{check_app_address_available, get_app_names, get_num_app, init_app_cx};
use crate::timer::get_time;
use core::cell::RefCell;
use lazy_static::*;
use switch::__switch;
use task::{Stride, TaskControlBlock, TaskStatus, BIG_STRIDE};

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let labels = get_app_names();
        let mut tasks = [TaskControlBlock {
            task_cx_ptr: 0,
            task_status: TaskStatus::UnInit,
            task_name: "",
            task_stride: Stride { value: 0 },
            task_priority: 16,
            task_awake_time: 0,
            task_elapse_time: 0,
            task_last_switch_time: 0,
        }; MAX_APP_NUM];
        for i in 0..num_app {
            tasks[i].task_cx_ptr = init_app_cx(i) as *const _ as usize;
            tasks[i].task_status = TaskStatus::Ready;
            tasks[i].task_name = labels[i];
        }
        TaskManager {
            num_app,
            inner: RefCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
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

pub fn validate_app_address(address: usize) -> bool {
    check_app_address_available(TASK_MANAGER.get_current_app(), address)
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_elapse_time +=
            get_time() - inner.tasks[current].task_last_switch_time;
        if inner.tasks[current].task_elapse_time > MAX_APP_LIFETIME_CLOCK {
            inner.tasks[current].task_status = TaskStatus::Exited;
            core::mem::drop(inner);
            warn!(
                "[kernel] Force stop the long lifetime app({}) which maybe dead loop",
                self.inner.borrow().tasks[current].task_name
            );
            info!(
                "[kernel] {} executed for {}ms",
                self.inner.borrow().tasks[current].task_name,
                self.inner.borrow().tasks[current].task_elapse_time / (CLOCK_FREQ / MSEC_PER_SEC)
            );
        } else {
            inner.tasks[current].task_status = TaskStatus::Ready;
        }
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_elapse_time +=
            get_time() - inner.tasks[current].task_last_switch_time;
        inner.tasks[current].task_status = TaskStatus::Exited;
        core::mem::drop(inner);
        info!(
            "[kernel] {} executed for {}ms",
            self.inner.borrow().tasks[current].task_name,
            self.inner.borrow().tasks[current].task_elapse_time / (CLOCK_FREQ / MSEC_PER_SEC)
        );
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
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
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.tasks[next].task_stride.value += BIG_STRIDE / inner.tasks[next].task_priority;
            inner.tasks[next].task_last_switch_time = get_time();
            inner.current_task = next;

            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            debug!(
                "[kernel] run next task {} with stride={}",
                self.inner.borrow().tasks[next].task_name,
                self.inner.borrow().tasks[next].task_stride.value,
            );
            unsafe {
                __switch(current_task_cx_ptr2, next_task_cx_ptr2);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn run_first_task(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.tasks[0].task_status = TaskStatus::Running;
        inner.tasks[0].task_last_switch_time = get_time();
        core::mem::drop(inner);
        let next_task_cx_ptr2 = self.inner.borrow().tasks[0].get_task_cx_ptr2();
        let _unused: usize = 0;
        debug!(
            "[kernel] run first task {}",
            self.inner.borrow().tasks[0].task_name
        );
        unsafe {
            __switch(&_unused as *const _, next_task_cx_ptr2);
        }
    }

    fn get_current_app(&self) -> usize {
        self.inner.borrow().current_task
    }

    fn update_current_task_priority(&self, prio: u16) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_priority = prio;
    }

    fn current_sleep_for_ticks(&self, ticks: usize) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_awake_time = get_time() + ticks;
    }
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
