extern crate libc;
use libc::c_float;

#[link(name = "libnovelrt")]
extern {
    fn lib_novel_rt_run_on_update(subscriber: fn(c_float) -> ());
}

pub fn run_on_update(subscriber: fn(c_float) -> ()) {
    unsafe {
        lib_novel_rt_run_on_update(subscriber);
    };
}

pub fn test_call() {
    run_on_update(|deltaTime| {
        println!("deltaTime is {}", deltaTime);
    });
}