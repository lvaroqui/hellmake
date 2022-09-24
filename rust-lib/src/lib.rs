use lazy_static::lazy_static;
use libc::{
    c_char, c_int, c_uint, mode_t, stat as stat_struct, statx as statx_struct, FILE, O_CREAT,
    O_TMPFILE, RTLD_NEXT,
};
use std::ffi::CStr;

use libc_func_wrapper::wrap_libc_func;

#[wrap_libc_func]
fn open(path: *const c_char, oflag: c_int, mode: mode_t) -> c_int {
    println!("Open {:?} {:?} {:?}", CStr::from_ptr(path), oflag, mode);
}

#[wrap_libc_func]
fn open64(path: *const c_char, oflag: c_int, mode: mode_t) -> c_int {
    let mut mode = mode;
    if (oflag & O_CREAT) != 0 || (oflag & O_TMPFILE) != 0 {
        println!("O_CREAT or O_TMPFILE");
    } else {
        mode = 0;
    }
    println!("Open64 {:?} {:0b} {:0o}", CStr::from_ptr(path), oflag, mode);
}

#[wrap_libc_func]
fn openat(dirfd: c_int, path: *const c_char, oflag: c_int, mode: mode_t) -> c_int {
    println!(
        "OpenAt {:?} {:?} {:?} {:?}",
        dirfd,
        CStr::from_ptr(path),
        oflag,
        mode
    );
}

#[wrap_libc_func]
fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE {
    println!(
        "FOpen {:?} {:?}",
        CStr::from_ptr(path),
        CStr::from_ptr(mode)
    );
}

#[wrap_libc_func]
fn fopen64(path: *const c_char, mode: *const c_char) -> *mut FILE {
    println!(
        "FOpen64 {:?} {:?}",
        CStr::from_ptr(path),
        CStr::from_ptr(mode)
    );
}

#[wrap_libc_func]
fn statx(
    dirfd: c_int,
    pathname: *const c_char,
    flags: c_int,
    mask: c_uint,
    statxbuf: *mut statx_struct,
) -> c_int {
    println!("Statx {:?}", CStr::from_ptr(pathname));
}

#[wrap_libc_func]
fn stat(path: *const c_char, buf: *mut stat_struct) -> c_int {
    println!("Stat {:?}", CStr::from_ptr(path));
}

#[wrap_libc_func]
fn lstat(path: *const c_char, buf: *mut stat_struct) -> c_int {
    println!("LStat {:?}", CStr::from_ptr(path));
}
