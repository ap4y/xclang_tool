use libc::{c_void, c_uint};
use std::{c_str, fmt};

use ffi::{clang_getCString};

pub struct CXString {
    data: *c_void,
    private_flags: c_uint
}

impl fmt::Show for CXString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = unsafe {
            let buf = clang_getCString(*self);
            let c_str = c_str::CString::new(buf, false);
            if c_str.is_null() { return f.buf.write("".as_bytes()); }
            let cast = c_str.as_str();

            match cast {
                None => ~"",
                _ => cast.unwrap().to_str()
            }
        };

        f.buf.write(str.as_bytes())
    }
}
