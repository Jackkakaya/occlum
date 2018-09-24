use sgx_types::*;

use process;
use std::ffi::CStr; // a borrowed C string
use std::collections::HashMap;
// Use the internal syscall wrappers from sgx_tstd
//use std::libc_fs as fs;
//use std::libc_io as io;

/*
#[no_mangle]
pub unsafe extern "C" fn sys_open(path: * const c_char, flags: c_int, mode: c_int) -> c_int {
    fs::open64(path, flags, mode)
}

#[no_mangle]
pub unsafe extern "C" fn sys_close(fd: c_int) -> c_int {
    io::close(fd)
}

#[no_mangle]
pub unsafe extern "C" fn sys_read(fd: c_int, buf: * mut c_void, size: size_t) -> ssize_t {
    io::read(fd, buf, size)
}
*/

#[no_mangle]
pub extern "C" fn rusgx_write(fd: c_int, buf: * const c_void, size: size_t) -> ssize_t {
    let str_from_c = unsafe {
        CStr::from_ptr(buf as * const i8).to_string_lossy().into_owned()
    };
    println!("rusgx_write: {}", str_from_c);
    size as ssize_t
}

#[no_mangle]
pub extern "C" fn rusgx_getpid() -> c_uint
{
    process::do_getpid()
}

#[no_mangle]
pub extern "C" fn rusgx_exit(status: i32)
{
    process::do_exit(status);
}

#[no_mangle]
pub extern "C" fn rusgx_spawn(_child_pid: *mut c_int, path: *const c_char,
    argv: *const *const c_char, envp: *const *const c_char) -> c_int
{
    let mut ret = 0;
    let mut child_pid = 0;
    let path_str = unsafe {
        CStr::from_ptr(path as * const i8).to_string_lossy().into_owned()
    };
    if process::do_spawn(&mut child_pid, &path_str) != Ok(()) {
        ret = -1;
    }
    unsafe {
        *_child_pid = child_pid as c_int;
    }
    ret
}

#[no_mangle]
pub extern "C" fn rusgx_wait4(child_pid: c_int, _exit_code: *mut c_int,
    options: c_int/*, rusage: *mut Rusage*/) -> c_int
{
    let mut exit_code = 0;
    process::do_wait4(child_pid as u32, &mut exit_code);
    unsafe {
        *_exit_code = exit_code;
    }
    0
}