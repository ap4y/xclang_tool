use libc::{c_void, c_uint};
use std::ptr;

use ffi::{clang_getSpellingLocation, clang_getExpansionLocation, clang_getFileName};

/**
 * Source Location
 **/
pub struct SourceLocation {
    file:   ~str,
    line:   uint,
    column: uint,
    offset: uint
}

/**
 * CXSourceLocation
 **/

pub struct CXSourceLocation {
    ptr_data0: *c_void,
    ptr_data1: *c_void,
    int_data:  c_uint
}

impl CXSourceLocation {
    pub fn spelling_location(&self) -> SourceLocation {
        let file   = ptr::null();
        let line   = -1;
        let column = -1;
        let offset = -1;

        unsafe { clang_getSpellingLocation(*self, &file, &line, &column, &offset) };
        let file_name = unsafe { clang_getFileName(file) };

        SourceLocation {
            file:   file_name.to_str(),
            line:   line as uint,
            column: column as uint,
            offset: offset as uint
        }
    }

    pub fn expansion_location(&self) -> SourceLocation {
        let file   = ptr::null();
        let line   = -1;
        let column = -1;
        let offset = -1;

        unsafe { clang_getExpansionLocation(*self, &file, &line, &column, &offset) };
        let file_name = unsafe { clang_getFileName(file) };

        SourceLocation {
            file:   file_name.to_str(),
            line:   line as uint,
            column: column as uint,
            offset: offset as uint
        }
    }
}
