use std::{ffi::CStr, os::raw::c_int};

use pmapi_sys::pmErrStr;

pub fn get_error(sts: c_int) -> String {
    unsafe {
        let err_str = pmErrStr(sts);
        let c_str = CStr::from_ptr(err_str);
        c_str.to_string_lossy().into_owned()
    }
}
