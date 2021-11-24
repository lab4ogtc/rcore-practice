use core::cmp::Ordering::{self, Greater, Less};
use crate::config::{kernel_stack_position, TRAP_CONTEXT};
use crate::mm::address::{PhysPageNum, VirtAddr};
use crate::mm::memory_set::{KERNEL_SPACE, MapPermission, MemorySet};
use crate::task::TaskContext;
use crate::trap::{trap_handler, TrapContext};

pub const BIG_STRIDE: u16 = 65535; // u16::MAX
const STRIDE_REVERSE: u16 = BIG_STRIDE / 2;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

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

pub struct TaskControlBlock {
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub task_name: &'static str,
    pub task_stride: Stride,
    pub task_priority: u16,
    pub task_awake_time: usize,
    pub task_elapse_time: usize,
    pub task_last_switch_time: usize,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize, app_name: &'static str) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let task_status = TaskStatus::Ready;
        // map a kernel-stack in kernel space
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE
            .exclusive_access()
            .insert_framed_area(
                kernel_stack_bottom.into(),
                kernel_stack_top.into(),
                MapPermission::R | MapPermission::W,
            );
        let task_control_block = Self {
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            task_status,
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
            task_name: app_name,
            task_stride: Stride { value: 0 },
            task_priority: 16,
            task_awake_time: 0,
            task_elapse_time: 0,
            task_last_switch_time: 0,
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
}
