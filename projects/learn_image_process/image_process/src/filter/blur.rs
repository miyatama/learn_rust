use image::{DynamicImage, GenericImageView, Rgba};
use log::debug;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn apply(img: DynamicImage) -> DynamicImage {
    debug!("start blur::apply");
    let blur_filter: [[f32; 3]; 3] = [
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
    ];
    apply_filter(img, blur_filter)
}

#[allow(dead_code)]
pub fn apply_multi(img: DynamicImage) -> DynamicImage {
    debug!("start blur::apply_multi");
    let blur_filter: [[f32; 3]; 3] = [
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
    ];
    apply_filter_multi(img, blur_filter)
}

fn apply_filter(img: DynamicImage, filter: [[f32; 3]; 3]) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut result_pixels: Vec<Rgba<u8>> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let filtered_pixel = apply_filter_at_pixel(&img, x, y, &filter);
            result_pixels.push(filtered_pixel);
        }
    }
    let result_pixels = result_pixels
        .into_iter()
        .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
        .flatten()
        .collect::<Vec<u8>>();
    let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
    DynamicImage::ImageRgba8(result_img)
}

#[derive(Debug, PartialEq)]
struct LocatePixel {
    index: u32,
    pixel: Rgba<u8>,
}

impl PartialOrd for LocatePixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.partial_cmp(&other.index).unwrap())
    }
}
fn apply_filter_multi(img: DynamicImage, filter: [[f32; 3]; 3]) -> DynamicImage {
    let (width, height) = img.dimensions();
    debug!("start convert to vec");
    let mut pixel_data: Vec<Vec<u8>> = vec![];
    let mut position_data: Vec<(u32, u32)> = vec![];
    for y in 0..height {
        for x in 0..width {
            pixel_data.push(img.get_pixel(x, y).0.to_vec());
            position_data.push((x, y));
        }
    }

    debug!("start create thread");
    let pixel_data = Arc::new(Mutex::new(pixel_data));
    let mut handles: Vec<_> = position_data
        .into_iter()
        .map(|(x, y)| {
            thread::spawn({
                let pixel_data = Arc::clone(&pixel_data);
                move || {
                    let pixel_data = pixel_data.lock().unwrap();
                    apply_filter_at_pixel_multi(&pixel_data, x, y, width, height, &filter)
                }
            })
        })
        .collect();

    debug!("start running thread");
    let mut result_pixels: Vec<LocatePixel> = vec![];
    while handles.len() > 0 {
        let handle = handles.remove(0);
        result_pixels.push(handle.join().unwrap());
    }
    debug!("start creating picture");
    result_pixels.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let result_pixels = result_pixels
        .into_iter()
        .map(|data| data.pixel)
        .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
        .flatten()
        .collect::<Vec<u8>>();
    let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
    DynamicImage::ImageRgba8(result_img)
}

fn apply_filter_at_pixel(img: &DynamicImage, x: u32, y: u32, filter: &[[f32; 3]; 3]) -> Rgba<u8> {
    let mut sum_red: f32 = 0.0;
    let mut sum_green: f32 = 0.0;
    let mut sum_blue: f32 = 0.0;
    for j in 0..3 {
        for i in 0..3 {
            let px = x + i;
            let py = y + j;
            if img.in_bounds(px, py) {
                let pixel = img.get_pixel(px, py).0;
                let filter_value = filter[j as usize][i as usize];
                sum_red += pixel[0] as f32 * filter_value;
                sum_green += pixel[1] as f32 * filter_value;
                sum_blue += pixel[2] as f32 * filter_value;
            }
        }
    }
    Rgba([
        sum_red.round() as u8,
        sum_green.round() as u8,
        sum_blue.round() as u8,
        255,
    ])
}

fn apply_filter_at_pixel_multi(
    pixel_data: &Vec<Vec<u8>>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    filter: &[[f32; 3]; 3],
) -> LocatePixel {
    let mut sum_red: f32 = 0.0;
    let mut sum_green: f32 = 0.0;
    let mut sum_blue: f32 = 0.0;
    for j in 0..3 {
        for i in 0..3 {
            let px = x + i;
            let py = y + j;
            if px < width && py < height {
                let pixel = &pixel_data[(py * width + px) as usize];
                let filter_value = filter[j as usize][i as usize];
                sum_red += pixel[0] as f32 * filter_value;
                sum_green += pixel[1] as f32 * filter_value;
                sum_blue += pixel[2] as f32 * filter_value;
            }
        }
    }
    LocatePixel {
        index: y * width + x,
        pixel: Rgba([
            sum_red.round() as u8,
            sum_green.round() as u8,
            sum_blue.round() as u8,
            255,
        ]),
    }
}
