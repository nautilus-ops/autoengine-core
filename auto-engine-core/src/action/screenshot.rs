use std::io::Cursor;
use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use screenshots::image::{DynamicImage, ImageFormat};
use screenshots::Screen;

pub fn capture_to_tiff_bytes() -> Result<Vec<u8>, String> {
    let screen = Screen::all().map_err(|e| {e.to_string()})?.into_iter().next().unwrap();
    let image = screen.capture().map_err(|e| {e.to_string()})?;

    let dynamic_image = DynamicImage::ImageRgba8(image);

    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    dynamic_image.write_to(&mut cursor, ImageFormat::Tiff).map_err(|e| {e.to_string()})?;
    Ok(bytes)
}

// imgcodecs::IMREAD_COLOR | imgcodecs::IMREAD_GRAYSCALE
pub fn capture_to_mat(imread_type: i32) -> Result<Mat, String> {
    let bytes = capture_to_tiff_bytes()?;

    let screen = Vector::from_slice(&bytes);
    let screen_img = imgcodecs::imdecode(&screen, imread_type)
        .map_err(|err| err.to_string())?;

    Ok(screen_img)
}
