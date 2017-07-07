extern crate libc;

use std::slice;
use libc::{c_int, c_uchar, c_void, close, pipe, read, write};

#[cfg(not(target_os="windows"))]
extern "C" {
    fn fork () -> c_int;
    fn kill (pid: c_int, sig: c_int) -> c_int;
}

pub fn new_pipe() -> [c_int; 2] {
    let mut fds : [c_int; 2] = [0; 2];
    #[cfg(target_os="windows")]
    unsafe { pipe(fds.as_mut_ptr(), 2048, libc::O_BINARY) };
    #[cfg(not(target_os="windows"))]
    unsafe { pipe(fds.as_mut_ptr()) };
    fds
}

pub fn fork_process() -> c_int {
    #[cfg(target_os="windows")]
    return 1;
    #[cfg(not(target_os="windows"))]
    return unsafe { fork() };
}

pub fn kill_process(pid: c_int) -> c_int {
    #[cfg(target_os="windows")]
    return 1;
    #[cfg(not(target_os="windows"))]
    return unsafe { kill(pid, 9) };
}

pub fn set_non_blocking(fd: c_int) {
    #[cfg(not(target_os="windows"))]
    unsafe { libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) };
}

pub fn close_fd(fd: c_int) {
    unsafe { close(fd) };
}

pub fn send_message(fd: c_int, command: &str) {
    let mut arr = ::neovim::Array::new();
    arr.add_string(command);
    let msg = ::neovim::serialize_message(1, "vim_command", &arr);
    let msg_ref: &str = msg.as_ref();
    let msg_ptr = msg_ref.as_ptr() as *const c_void;
    #[cfg(target_os="windows")]
    let len = msg.len() as libc::c_uint;
    #[cfg(not(target_os="windows"))]
    let len = msg.len() as libc::size_t;
    unsafe { write(fd, msg_ptr, len) };
}

pub fn recv_message(fd: c_int) -> Option<::neovim::Array> {
    let mut buf : [c_uchar; 1024] = [0; 1024];
    let n = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, 1024) };
    if n < 0 {
        return None;
    }
    unsafe {
        let v = slice::from_raw_parts(buf.as_ptr(), n as usize).to_vec();
        let s = String::from_utf8_unchecked(v);
        Some(::neovim::deserialize_message(&s))
    }
}
