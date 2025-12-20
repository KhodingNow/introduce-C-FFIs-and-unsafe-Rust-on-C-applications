use libc::{c_char, c_int};
use std::ffi::CStr;

/*
#[no_mangle]

pub extern "C" fn solve(line: *const c_char, solution: *mut c_int) -> c_int {
    0
}
 */

#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}
