use cmake::Config;

fn main() {
    let dst = cmake::build("NovelRT");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=libnovelrt");
}