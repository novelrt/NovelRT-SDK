extern crate NovelRusT;

use NovelRusT::{create_runner, run_on_update, run_novel};
use NovelRusT::GeoVector::*;
use NovelRusT::NovelRunner;

#[no_mangle]
pub extern "C" fn test_subscriber(delta_time: f32) {
    println!("test subscriber 1 with deltatime: {}", delta_time)
}

fn main() {
    std::env::set_var("MESA_GL_VERSION_OVERRIDE", "4.2");
    let geo_vector = create_geo_vector(&mut 14,&mut 0);
    println!("{}", geo_vector.get_x());
    let runner = create_runner(0);
    run_on_update(runner, test_subscriber);
    run_novel(runner)
}
