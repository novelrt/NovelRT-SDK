extern crate libc;
use libc::c_int;

extern "C" {
    pub fn runNovel(display_number: c_int);
}
