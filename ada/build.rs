use std::env;

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
        .cpp(true);
    let compile_target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS");
    let compile_target_env = env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV");
    if !(compile_target_os == "windows" && compile_target_env == "msvc") {
        build.compiler("clang++");
        build.cpp_set_stdlib("c++").flag("-std=c++17");
        println!("cargo:rustc-link-lib=c++");
    } else {
        build.flag("/std:c++17").static_crt(true);
    }

    build.compile("ada");
}
