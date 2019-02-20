extern crate libc;
use libc::{c_int, c_float};

extern "C" {
    pub fn createRunner(display_number: c_int) -> *mut NovelRunner_t;
    pub fn runOnUpdate(runner: *mut NovelRunner_t, subscriber: extern "C" fn (c_float) -> ());
    pub fn runNovel(runner: *mut NovelRunner_t);
}

#[repr(C)]
pub struct NovelRunner_t;
