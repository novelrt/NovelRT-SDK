#[macro_use]
extern crate NovelRusT;

use NovelRusT::GeoVector::*;
use NovelRusT::NovelRunner::*;

fn test_subscriber(delta_time: f32) {
    println!("test subscriber 1 with deltatime: {}", delta_time)
}

fn main() {
    std::env::set_var("MESA_GL_VERSION_OVERRIDE", "4.2");
    let geo_vector = create_geo_vector(&mut 14,&mut 0);
    println!("{}", geo_vector.get_x());
    let mut runner = create_runner(0);
    runner.run_on_update(test_subscriber);
    runner.run_novel()
}
