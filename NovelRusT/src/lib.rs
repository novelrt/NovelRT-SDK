extern crate NovelRT_sys;
use NovelRT_sys::runNovel;
pub fn run_novel(display_number: i32) {
    unsafe {
        runNovel(display_number);
    };
}
