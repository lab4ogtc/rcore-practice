mod fs;
mod process;

use fs::*;
use process::*;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_SLEEP: usize = 101;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_SET_PRIORITY: usize = 140;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_MMAP: usize = 222;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    trace!("syscall: code={}, args=[{:#x}, {:#x}, {:#x}]", syscall_id, args[0], args[1], args[2]);
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_SLEEP => sys_sleep(args[0]),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_SET_PRIORITY => sys_set_priority(isize::from_ne_bytes(args[0].to_ne_bytes())),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_MUNMAP => sys_munmap(args[0], args[1]),
        SYSCALL_MMAP => sys_mmap(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
