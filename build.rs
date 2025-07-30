fn main() {
    cxx_build::bridge("src/bridge.rs")
        .file("source/example.cpp")
        .file("source/reverse.cpp")
        .include("source")
        .std("c++20")
        .compile("rust_cxx");

    println!("cargo:rerun-if-changed=source/example.h");
    println!("cargo:rerun-if-changed=source/example.cpp");
    println!("cargo:rerun-if-changed=source/reverse.h");
    println!("cargo:rerun-if-changed=source/reverse.cpp");
    println!("cargo:rerun-if-changed=src/bridge.rs");
}
