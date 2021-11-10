use crate::task::validate_app_address;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!(
        "[kernel] trigger sys_write(fd:{}, buf:{:?}, len:{})",
        fd,
        buf,
        len
    );
    let addr_start: usize = buf as usize;
    let addr_end: usize = addr_start + len;
    if !validate_app_address(addr_start) || !validate_app_address(addr_end) {
        warn!(
            "[kernel] Invalid address for kernel writing on {:#x}-{:#x}",
            addr_start, addr_end
        );
        return -1;
    }

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            error!("Unsupported fd in sys_write!");
            -1
        }
    }
}
