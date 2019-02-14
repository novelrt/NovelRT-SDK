extern crate libc;
use libc::c_int;

#[link(name = "libnovelrt")]
extern {
    fn runNovel(display_number: c_int);
}

pub fn run_novel(display_number: i32) {
    unsafe {
        runNovel(display_number);
    };
}
