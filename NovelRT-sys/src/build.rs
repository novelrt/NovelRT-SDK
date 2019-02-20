fn main() {
    println!("cargo:rustc-link-lib=dylib=NovelRTLib");
    println!("cargo:rustc-link-search=native=../lib")
}