use NovelRT_sys::NovelRunnerWrapper;
use NovelRT_sys::{createRunner, runOnUpdate, runNovel};
use std::mem::transmute;

pub struct NovelRunner {
    _wrapper: *mut NovelRunnerWrapper
}

pub fn create_runner(display_number: i32) -> NovelRunner {
    let runner: *mut NovelRunnerWrapper;
    unsafe {
        runner = createRunner(display_number)
    }
    NovelRunner {
        _wrapper: runner
    }
}

impl NovelRunner {
    pub fn run_novel(&self) {
        unsafe {
            runNovel(self._wrapper)
        }
    }

    pub fn run_on_update(&self, subscriber: fn(f32))
    {
        unsafe {

            let c_fn: extern "C" fn(f32) = |x| println!("{}");
            runOnUpdate(self._wrapper, c_fn)
        }
    }
}

