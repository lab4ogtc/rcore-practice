use core::cmp::Ordering;
use core::cmp::Ordering::{Greater, Less};

pub const BIG_STRIDE: u16 = 65535; // u16::MAX
const STRIDE_REVERSE: u16 = BIG_STRIDE / 2;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Copy, Clone)]
pub struct Stride {
    pub value: u16,
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(Less) => {
                if other.value - self.value > STRIDE_REVERSE {
                    Some(Greater)
                } else { Some(Less) }
            }
            Some(Greater) => {
                if self.value - other.value > STRIDE_REVERSE {
                    Some(Less)
                } else { Some(Greater) }
            }
            other => { other }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_cx_ptr: usize,
    pub task_status: TaskStatus,
    pub task_name: &'static str,
    pub task_stride: Stride,
    pub task_priority: u16,
    pub task_awake_time: usize,
    pub task_elapse_time: usize,
    pub task_last_switch_time: usize,
}

impl TaskControlBlock {
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }
}
