extern crate NovelRusT;

use NovelRusT::{create_runner, run_on_update, run_novel};
use NovelRusT::NovelRunner_t;

#[no_mangle]
pub extern "C" fn test_subscriber(delta_time: f32) {
    println!("test subscriber 1 with deltatime: {}", delta_time)
}

fn main() {
    let lib_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/../lib";
    std::env::set_var("MESA_GL_VERSION_OVERRIDE", "4.2");
    std::env::set_var("LD_LIBRARY_PATH", lib_dir);

    let runner = create_runner(0);
    run_on_update(runner, test_subscriber);
    run_novel(runner)
}
