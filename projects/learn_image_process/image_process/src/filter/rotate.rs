use crate::util::custom_error::CustomError;
use image::{DynamicImage, GenericImageView, Rgba};

pub fn apply(
    img: DynamicImage,
    degree: i32,
) -> Result<Result<DynamicImage, CustomError>, anyhow::Error> {
    log::debug!("rotate::apply");
    let rad = degree as f64 * std::f64::consts::PI / 180f64;
    let (width, height) = img.dimensions();
    let rotated_rect = vec![
        (0f64, 0f64),
        (0f64, height as f64),
        (width as f64, 0f64),
        (width as f64, height as f64),
    ]
    .iter()
    .map(|(x, y)| (x * rad.cos() - y * rad.sin(), x * rad.sin() + y * rad.cos()))
    .collect::<Vec<(f64, f64)>>();
    let minx = rotated_rect
        .iter()
        .map(|(x, _)| x)
        .fold(0.0 / 0.0, |a, b| b.min(a));
    let miny = rotated_rect
        .iter()
        .map(|(_, y)| y)
        .fold(0.0 / 0.0, |a, b| b.min(a));
    let maxx = rotated_rect
        .iter()
        .map(|(x, _)| x)
        .fold(0.0 / 0.0, |a, b| b.max(a));
    let maxy = rotated_rect
        .iter()
        .map(|(_, y)| y)
        .fold(0.0 / 0.0, |a, b| b.max(a));

    let new_width = (maxx - minx) as u32;
    let new_height = (maxy - miny) as u32;
    let mut result_pixels: Vec<Rgba<u8>> = Vec::with_capacity((new_width * new_height) as usize);
    let result_pixels = result_pixels
        .into_iter()
        .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
        .flatten()
        .collect::<Vec<u8>>();
    let result_img = image::ImageBuffer::from_raw(new_width, new_height, result_pixels).unwrap();
    Ok(Ok(DynamicImage::ImageRgba8(result_img)))
}
