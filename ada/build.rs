fn main() {
    println!("cargo:rerun-if-changed=binding/ada.cpp");
    println!("cargo:rerun-if-changed=binding/ada.hpp");
    println!("cargo:rerun-if-changed=binding/ada-binding.cpp");
    println!("cargo:rerun-if-changed=binding/ada-binding.hpp");
    let mut build = cc::Build::new();
    build
        .file("./binding/ada.cpp")
        .file("./binding/ada-binding.cpp")
        .include("./binding/ada.hpp")
        .include("./binding/ada-binding.hpp")
        .cpp(true)
        .compiler("clang++")
        .flag("-std=c++17")
        .cpp_set_stdlib("c++")
        .compile("ada");
    println!("cargo:rustc-link-lib=c++");
}
