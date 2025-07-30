mod cxx;

use image::{GrayImage, ImageError};

fn load_grayscale_image(path: &str) -> Result<GrayImage, ImageError> {
    let img = image::open(path)?;
    Ok(img.into_luma8())
}

fn main() -> Result<(), ImageError> {
    let img = load_grayscale_image("kuma.jpg")?;
    let (width, height) = img.dimensions();
    let img_data = img.into_raw();

    let reversed = cxx::transform(&img_data);
    let img_reversed = GrayImage::from_raw(width, height, reversed).expect("Failed");

    img_reversed.save("kuma_reversed.png")?;

    Ok(())
}
