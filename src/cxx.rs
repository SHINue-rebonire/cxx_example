#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example.h");

        fn add(a: i32, b: i32) -> i32;
    }
}

pub use ffi::*;
