extern crate libc;
use libc::c_int;

#[link(name = "NovelRTLib")]
extern "C" {
    pub fn createRunner(display_number: c_int) -> *mut NovelRunner_t;
}

#[repr(C)]
pub struct NovelRunner_t;
