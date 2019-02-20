extern crate NovelRusT;

use NovelRusT::create_runner;
use NovelRusT::NovelRunner_t;
fn main() {
    let lib_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/../lib";
    std::env::set_var("MESA_GL_VERSION_OVERRIDE", "4.2");
    std::env::set_var("LD_LIBRARY_PATH", "../lib");

    let runner = create_runner(0);
}
