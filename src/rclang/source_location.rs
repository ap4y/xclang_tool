use libc::{c_void, c_uint};
use std::ptr;
use std::fmt;

use ffi::{clang_getSpellingLocation, clang_getExpansionLocation, clang_getFileName};

/**
 * Source Location
 **/
pub struct SourceLocation {
    pub file:   ~str,
    pub line:   uint,
    pub column: uint,
    pub offset: uint
}

impl fmt::Show for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self.file + ":" + self.line.to_str() + ":" + self.column.to_str();
        f.buf.write(str.as_bytes())
    }
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
