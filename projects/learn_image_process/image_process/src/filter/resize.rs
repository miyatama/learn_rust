use crate::util::custom_error::CustomError;
use image::{DynamicImage, GenericImageView, Rgba};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(dead_code)]
pub fn apply(
    img: DynamicImage,
    scale: f64,
) -> Result<Result<DynamicImage, CustomError>, anyhow::Error> {
    log::debug!("resize::apply");
    let (width, height) = img.dimensions();
    let new_width = (width as f64 * scale).trunc() as u32;
    let new_height = (height as f64 * scale).trunc() as u32;
    log::debug!(
        "change rect: ({}, {}) -> ({}, {})",
        width,
        height,
        new_width,
        new_height
    );
    let mut src_img_map: Vec<Vec<Rgba<u8>>> =
        vec![vec![Rgba([0u8, 0u8, 0u8, 0u8]); height as usize]; width as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            src_img_map[x as usize][y as usize] =
                Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255]);
        }
    }
    let mut result_pixels: Vec<Rgba<u8>> = Vec::with_capacity((new_width * new_height) as usize);
    let mut hm: HashMap<usize, Rgba<u8>> = HashMap::new();
    let calc_new_pos = |x: f64, y: f64| ((x * scale).trunc() as u32, (y * scale).trunc() as u32);
    for y in 0..height {
        for x in 0..width {
            let (new_x, new_y) = calc_new_pos(x as f64, y as f64);
            let pos = (new_x as f64 + new_y as f64 * new_width as f64) as usize;
            let pixel = src_img_map[x as usize][y as usize];
            hm.insert(
                pos,
                Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255]),
            );
        }
    }
    log::debug!("finish creating position map");

    for y in 0..new_height {
        for x in 0..new_width {
            let pos = (x + y * new_width) as usize;
            match hm.get(&pos) {
                Some(pixel) => {
                    result_pixels.push(*pixel);
                }
                None => {
                    result_pixels.push(Rgba([0u8, 0u8, 0u8, 0u8]));
                }
            }
        }
    }
    log::debug!("finish setting moved color");

    let edit_pixels: Arc<Mutex<Vec<Rgba<u8>>>> = Arc::new(Mutex::new(result_pixels.clone()));
    let correction_base_pixels: Arc<Mutex<Vec<Vec<Rgba<u8>>>>> =
        Arc::new(Mutex::new(src_img_map.clone()));
    let mut handles = vec![];
    for y in 0..new_height {
        for x in 0..new_width {
            let pos = (x + y * new_width) as usize;
            let color = result_pixels[pos];
            if color[3] == 0 {
                let edit_pixels = Arc::clone(&edit_pixels);
                let base_pixels = Arc::clone(&correction_base_pixels);
                handles.push(thread::spawn(move || {
                    // 元々の位置の情報を参照する
                    let src_x = (x as f64 / scale).trunc() as usize;
                    let src_y = (y as f64 / scale).trunc() as usize;
                    let width = width as usize;
                    let height = height as usize;

                    // 上下左右の中間色を設定する
                    let mut target_pos = vec![];
                    target_pos.push((src_x as usize, src_y as usize));
                    if src_x > 0 {
                        target_pos.push(((src_x - 1) as usize, src_y as usize));
                    }
                    if src_x < (width - 1) {
                        target_pos.push(((src_x + 1) as usize, src_y as usize));
                    }
                    if src_y > 0 {
                        target_pos.push((src_x, src_y - 1));
                    }
                    if src_y < (height - 1) {
                        target_pos.push((src_x, src_y + 1));
                    }

                    let base_pixels = match base_pixels.lock() {
                        Ok(pixels) => pixels,
                        Err(e) => {
                            log::error!("get base pixel error: {}", e);
                            return;
                        }
                    };
                    let target_colors = target_pos
                        .into_iter()
                        .map(|(x, y)| base_pixels[x][y])
                        .collect::<Vec<_>>();
                    let color =
                        target_colors
                            .iter()
                            .fold((0f64, 0f64, 0f64, 0f64), |sum, color| {
                                (
                                    sum.0 + color[0] as f64,
                                    sum.1 + color[1] as f64,
                                    sum.2 + color[2] as f64,
                                    sum.3 + 1.0f64,
                                )
                            });
                    let (r, g, b) = (
                        (color.0 / color.3) as u8,
                        (color.1 / color.3) as u8,
                        (color.2 / color.3) as u8,
                    );
                    let rgba = Rgba([r, g, b, 255u8]);
                    let mut pixels = edit_pixels.lock().unwrap();
                    pixels[pos] = rgba;
                }));
            }
        }
    }
    log::debug!("set pixel handle count: {}", handles.len());
    for handle in handles {
        handle.join().unwrap();
    }
    let pixels = edit_pixels.lock().unwrap();
    let result_pixels = pixels
        .clone()
        .into_iter()
        .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
        .flatten()
        .collect::<Vec<u8>>();
    let result_img = image::ImageBuffer::from_raw(new_width, new_height, result_pixels).unwrap();
    Ok(Ok(DynamicImage::ImageRgba8(result_img)))
}
