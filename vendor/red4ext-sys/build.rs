use std::path::PathBuf;

fn main() {
    let includes: &[PathBuf] = &[
        PathBuf::from("cpp").join("RED4ext.SDK").join("include"),
        PathBuf::from("cpp").join("glue"),
    ];

    let mut build = cxx_build::bridge("src/lib.rs");
    build.includes(includes);

    // MSVC uses /std:c++20, GCC/Clang uses -std=c++20
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
        build.flag("/std:c++20");
    } else {
        build.flag("-std=c++20");
    }

    build.compile("red4ext-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/glue/glue.hpp");
}
