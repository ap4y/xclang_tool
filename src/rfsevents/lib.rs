#![crate_id = "rfsevents#0.1"]
#![desc = "Rust binding for fsevents"]
#![license = "MIT"]
#![crate_type = "lib"]

extern crate std;
extern crate collections;
extern crate libc;

use libc::{c_char, c_double, c_void, c_uint, c_int, c_ulong};
use std::{str, cast, ptr};
use collections::enum_set::CLike;

/**
 * Types
 **/

pub type FSEventStreamRef = *c_void;
pub type FSEventStreamEventId = c_ulong;

#[allow(non_camel_case_types)]
#[repr(uint)]
pub enum FSEventStreamEventFlags {
    kFSEventStreamEventFlagNone = 0x00000000,
    kFSEventStreamEventFlagMustScanSubDirs = 0x00000001,
    kFSEventStreamEventFlagUserDropped = 0x00000002,
    kFSEventStreamEventFlagKernelDropped = 0x00000004,
    kFSEventStreamEventFlagEventIdsWrapped = 0x00000008,
    kFSEventStreamEventFlagHistoryDone = 0x00000010,
    kFSEventStreamEventFlagRootChanged = 0x00000020,
    kFSEventStreamEventFlagMount = 0x00000040,
    kFSEventStreamEventFlagUnmount = 0x00000080,
    kFSEventStreamEventFlagItemCreated = 0x00000100,
    kFSEventStreamEventFlagItemRemoved = 0x00000200,
    kFSEventStreamEventFlagItemInodeMetaMod = 0x00000400,
    kFSEventStreamEventFlagItemRenamed = 0x00000800,
    kFSEventStreamEventFlagItemModified = 0x00001000,
    kFSEventStreamEventFlagItemFinderInfoMod = 0x00002000,
    kFSEventStreamEventFlagItemChangeOwner = 0x00004000,
    kFSEventStreamEventFlagItemXattrMod = 0x00008000,
    kFSEventStreamEventFlagItemIsFile = 0x00010000,
    kFSEventStreamEventFlagItemIsDir = 0x00020000,
    kFSEventStreamEventFlagItemIsSymlink = 0x00040000
}
impl CLike for FSEventStreamEventFlags {
    fn to_uint(&self) -> uint { *self as uint }
    fn from_uint(v: uint) -> FSEventStreamEventFlags { unsafe { cast::transmute(v) } }
}

/**
 * Bindings
 **/

#[link(name = "RFSEvents")]
extern {
    pub fn createStream(path: *c_char,
                        latency: c_double,
                        target: *mut FSEventStream,
                        callback: extern fn(*mut FSEventStream, *c_char, c_uint, c_ulong)) -> FSEventStreamRef;

    pub fn scheduleStreamInRunLoop(stream: FSEventStreamRef) -> c_int;
    pub fn unscheduleStream(stream: FSEventStreamRef);
    pub fn destroyStream(stream: FSEventStreamRef);
}

/**
 * API
 **/

extern fn callback(target: *mut FSEventStream, path: *c_char, flags: c_uint, id: c_ulong) {
    let path_str = unsafe { str::raw::from_c_str(path) };
    unsafe { ((*target).callback)(Path::new(path_str), CLike::from_uint(flags as uint), id) };
}

pub struct FSEventStream<'a> {
    stream_ref: FSEventStreamRef,
    callback: |Path, FSEventStreamEventFlags, FSEventStreamEventId|:'a
}
impl<'a> FSEventStream<'a> {
    pub fn new(dir: &Path, latency: f64, cb: |Path, FSEventStreamEventFlags, FSEventStreamEventId|:'a) -> ~FSEventStream<'a> {

        let mut stream = ~FSEventStream {
            stream_ref: ptr::null(),
            callback: cb
        };

        let stream_ref = dir.with_c_str(|_dir| unsafe {
            createStream(_dir, latency, &mut *stream, callback)
        });

        stream.stream_ref = stream_ref;
        stream
    }

    pub fn schedule_stream(&self) -> bool {
        unsafe { scheduleStreamInRunLoop(self.stream_ref) > 0 }
    }

    pub fn unschedule_stream(&self) {
        unsafe { unscheduleStream(self.stream_ref); }
    }
}
#[unsafe_destructor]
impl<'a> Drop for FSEventStream<'a> {
    fn drop(&mut self) {
        unsafe { destroyStream(self.stream_ref); }
    }
}
