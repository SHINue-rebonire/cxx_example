fn main() {
    cxx_build::bridge("src/cxx.rs")
        .file("source/example.cpp")
        .file("source/reverse.cpp")
        .include("source")
        .flag_if_supported("-std=c++20")
        .compile("rust_cxx");

    println!("cargo:rerun-if-changed=source/example.h");
    println!("cargo:rerun-if-changed=source/example.cpp");
    println!("cargo:rerun-if-changed=source/reverse.h");
    println!("cargo:rerun-if-changed=source/reverse.cpp");
    println!("cargo:rerun-if-changed=src/cxx.rs");
}
