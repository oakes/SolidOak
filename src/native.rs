use libc::{c_int, c_uchar, c_void};
use libc::consts::os::extra::O_NONBLOCK;
use libc::consts::os::posix01::F_SETFL;
use libc::funcs::posix88::fcntl::fcntl;
use libc::funcs::posix88::unistd::{close, pipe, read, write};
use libc::types::os::arch::c95::size_t;

extern "C" {
    fn fork () -> c_int;
    fn kill (pid: c_int, sig: c_int) -> c_int;
}

pub fn new_pipe() -> [c_int; 2] {
    let mut fds : [c_int; 2] = [0; 2];
    unsafe { pipe(fds.as_mut_ptr()) };
    fds
}

pub fn fork_process() -> c_int {
    unsafe { fork() }
}

pub fn kill_process(pid: c_int) -> c_int {
    unsafe { kill(pid, 9) }
}

pub fn set_non_blocking(fd: c_int) {
    unsafe { fcntl(fd, F_SETFL, O_NONBLOCK) };
}

pub fn close_fd(fd: c_int) {
    unsafe { close(fd) };
}

pub fn send_message(fd: c_int, command: &str) {
    let mut arr = ::neovim::Array::new();
    arr.add_string(command);
    let msg = ::neovim::serialize_message(1, "vim_command", &arr);
    let msg_ptr = msg.as_slice().as_ptr() as *const c_void;
    unsafe { write(fd, msg_ptr, msg.len() as size_t) };
}

pub fn recv_message(fd: c_int) -> Option<::neovim::Array> {
    let mut buf : [c_uchar; 1024] = [0; 1024];
    let n = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, 1024) };
    if n < 0 {
        return None;
    }
    unsafe {
        let v = Vec::from_raw_buf(buf.as_ptr(), n as usize);
        let s = String::from_utf8_unchecked(v);
        Some(::neovim::deserialize_message(&s))
    }
}
