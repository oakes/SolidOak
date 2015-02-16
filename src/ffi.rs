pub use libc::{c_int, c_uchar, c_void};
pub use libc::consts::os::extra::O_NONBLOCK;
pub use libc::consts::os::posix01::F_SETFL;
pub use libc::funcs::posix88::fcntl::fcntl;
pub use libc::funcs::posix88::unistd::{close, pipe, read, write};
pub use libc::types::os::arch::c95::size_t;

extern "C" {
    pub fn fork () -> c_int;
    pub fn kill (pid: c_int, sig: c_int);
}

pub fn nvim_attach(fd: c_int) {
    let mut arr = ::neovim::Array::new();
    arr.add_integer(80);
    arr.add_integer(24);
    arr.add_boolean(true);
    let msg = ::neovim::serialize_message(1, "ui_attach", &arr);
    let msg_ptr = msg.as_slice().as_ptr() as *const c_void;
    unsafe { write(fd, msg_ptr, msg.len() as size_t) };
}

pub fn nvim_execute(fd: c_int, command: &str) {
    let mut arr = ::neovim::Array::new();
    arr.add_string(command);
    let msg = ::neovim::serialize_message(1, "vim_command", &arr);
    let msg_ptr = msg.as_slice().as_ptr() as *const c_void;
    unsafe { write(fd, msg_ptr, msg.len() as size_t) };
}

pub fn receive_message(fd: c_int) -> Option<::neovim::Array> {
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
