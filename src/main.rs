mod cxx;

use image::{GrayImage, ImageError};

fn load_grayscale_image(path: &str) -> Result<GrayImage, ImageError> {
    let img = image::open(path)?;
    Ok(img.into_luma8())
}

fn main() -> Result<(), ImageError> {
    let img = load_grayscale_image("kuma.jpg")?;
    let (width, height) = img.dimensions();
    // let img_data = img.into_raw();
    let empty_data: Vec<u8> = vec![]; // 空のデータ

    match cxx::transform(&empty_data) {
        Ok(reversed) => {
            let img_reversed = image::GrayImage::from_raw(width, height, reversed).expect("Failed");
            img_reversed.save("kuma_reversed.png")?;
        }
        Err(e) => println!("C++側エラー: {}", e),
    }

    Ok(())
}
