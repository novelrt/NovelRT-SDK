extern crate NovelRT_sys;
pub mod GeoVector;
use NovelRT_sys::{createRunner, runOnUpdate, runNovel};
pub use NovelRT_sys::NovelRunner;

pub fn create_runner(display_number: i32) -> *mut NovelRunner {
    unsafe {
        createRunner(display_number)
    }
}

pub fn run_on_update(runner: *mut NovelRunner, subscriber: extern "C" fn(f32) -> ()) {
    unsafe {
        runOnUpdate(runner, subscriber)
    }
}

pub fn run_novel(runner: *mut NovelRunner) {
    unsafe {
        runNovel(runner)
    }
}
