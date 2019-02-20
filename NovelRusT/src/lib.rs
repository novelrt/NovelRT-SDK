extern crate NovelRT_sys;
use NovelRT_sys::{createRunner, runOnUpdate, runNovel};
pub use NovelRT_sys::NovelRunner_t;

pub fn create_runner(display_number: i32) -> *mut NovelRunner_t {
    unsafe {
        createRunner(display_number)
    }
}

pub fn run_on_update(runner: *mut NovelRunner_t, subscriber: extern "C" fn(f32) -> ()) {
    unsafe {
        runOnUpdate(runner, subscriber)
    }
}

pub fn run_novel(runner: *mut NovelRunner_t) {
    unsafe {
        runNovel(runner)
    }
}
