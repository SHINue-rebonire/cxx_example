#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example.h");
        include!("reverse.h");

        fn add(a: i32, b: i32) -> i32;
        fn transform(img_data: &[u8]) -> Vec<u8>;
    }
}

pub use ffi::*;
