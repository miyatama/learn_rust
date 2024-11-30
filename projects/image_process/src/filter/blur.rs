use image::{DynamicImage, GenericImageView, Rgba};
use log::debug;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::cell::RefCell;

pub fn apply(img: DynamicImage) -> DynamicImage {
    debug!("start blur::apply");
    let blur_filter: [[f32; 3]; 3] = [
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
    ];
    apply_filter(img, blur_filter)
}

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

fn apply_filter_multi(img: DynamicImage, filter: [[f32; 3]; 3]) -> DynamicImage {
    let image_pointer = &img;
    thread_local! {
        static FILTER_BASE_IMAGE: RefCell<DynamicImage> = RefCell::new(image_pointer);
    }
    let (width, height) = img.dimensions();
    let result_pixels: Vec<Rgba<u8>> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width).into_par_iter().map(move |x| {
                FILTER_BASE_IMAGE
                    .with(|base_image| apply_filter_at_pixel(&base_image.borrow(), x, y, &filter))
            })
        })
        .collect();
    let result_pixels = result_pixels
        .into_iter()
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
