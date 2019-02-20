extern crate NovelRT_sys;
use NovelRT_sys::createRunner;
pub use NovelRT_sys::NovelRunner_t;
#[link(name = "NovelRTLib")]
pub fn create_runner(display_number: i32) -> *mut NovelRunner_t {
    unsafe {
        return createRunner(display_number);
    };
}
