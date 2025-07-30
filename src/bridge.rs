#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("reverse.h");

        fn transform(img_data: &[u8]) -> Result<Vec<u8>>;
    }
}

pub use ffi::*;
