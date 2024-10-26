use std::io::Cursor;

use image::{ImageBuffer, Rgba};
use rusty_tesseract::{
    image::{ImageFormat, ImageReader},
    Args, Image,
};
// use std::io::Write;

pub fn picture_ocr(
    img_buf: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<String, Box<dyn std::error::Error>> {
    
    let mut img_data = Vec::new();
    img_buf.write_to(&mut Cursor::new(&mut img_data), ImageFormat::Bmp)?;

    // let mut file = std::fs::File::create("output.bmp")?;
    // file.write_all(&img_data)?;

    let saved_data = std::fs::read("output.bmp")?;
    assert_eq!(img_data, saved_data, "Data mismatch!");

    let cursor = Cursor::new(img_data);

    let dynamic_image = ImageReader::new(cursor).with_guessed_format()?.decode()?;

    let img = Image::from_dynamic_image(&dynamic_image)?;

    let my_args = Args::default();

    let output = rusty_tesseract::image_to_string(&img, &my_args)?;

    Ok(output)
}
