use crate::util::custom_error::CustomError;
use image::{DynamicImage, GenericImageView, Rgba};
use std::collections::HashMap;

pub fn apply(
    img: DynamicImage,
    scale: f64,
) -> Result<Result<DynamicImage, CustomError>, anyhow::Error> {
    log::debug!("resize::apply");
    let (width, height) = img.dimensions();
    let new_width = (width as f64 * scale).trunc() as u32;
    let new_height = (height as f64 * scale).trunc() as u32;
    log::debug!("change rect: ({}, {}) -> ({}, {})", width, height, new_width, new_height);
    let mut result_pixels: Vec<Rgba<u8>> = Vec::with_capacity((new_width * new_height) as usize);
    let mut hm: HashMap<usize, Rgba<u8>> = HashMap::new();
    let calc_new_pos = |x: f64, y: f64| ((x * scale).trunc() as u32, (y * scale).trunc() as u32);
    for y in 0..height {
        for x in 0..width {
            let (new_x, new_y) = calc_new_pos(x as f64, y as f64);
            let pos = (new_x as f64 + new_y as f64 * new_width as f64) as usize;
            let pixel = img.get_pixel(x, y).0;
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
    let get_correction_color_pos =
        |x: u32, y: u32, direction: bool, end: u32, result_pixels: Vec<Rgba<u8>>| {
            if (direction && end == x) || (!direction && end == y) {
                return None;
            }
            let initial = if direction { x } else { y };
            let initial = if initial > end {
                initial - 1
            } else {
                initial + 1
            };
            for i in initial..end {
                let pos = if direction {
                    i + y * new_width
                } else {
                    x + i * new_width
                };
                let pos = pos as usize;
                if result_pixels[pos][3] != 0 {
                    return Some(result_pixels[pos]);
                }
            }
            None
        };

    for y in 0..new_height {
        for x in 0..new_width {
            let pos = (x + y * new_width) as usize;
            // 色が設定されていない場合は上下左右の色を使って補正する
            let color = result_pixels[pos];
            if color[3] == 0 {
                // log::debug!("setting correction color: {}", &pos);
                let correction_colors = vec![
                    get_correction_color_pos(x, y, true, 0, result_pixels.clone()),
                    get_correction_color_pos(x, y, true, new_width, result_pixels.clone()),
                    get_correction_color_pos(x, y, false, 0, result_pixels.clone()),
                    get_correction_color_pos(x, y, false, new_height, result_pixels.clone()),
                ];
                let color = correction_colors
                    .iter()
                    .filter(|color| color.is_some())
                    .map(|color| color.unwrap())
                    .fold((0f64, 0f64, 0f64, 0f64), |sum, color| {
                        (
                            sum.0 + color[0] as f64,
                            sum.1 + color[1] as f64,
                            sum.2 + color[2] as f64,
                            sum.3 + 1.0,
                        )
                    });
                let (r, g, b, count) = color;
                let rgba = Rgba([
                    (r / count) as u8,
                    (g / count) as u8,
                    (b / count) as u8,
                    255u8,
                ]);
                result_pixels[pos] = rgba;
            }
        }
    }
    // 色の補正
    let result_pixels = result_pixels
        .into_iter()
        .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
        .flatten()
        .collect::<Vec<u8>>();
    let result_img = image::ImageBuffer::from_raw(new_width, new_height, result_pixels).unwrap();
    Ok(Ok(DynamicImage::ImageRgba8(result_img)))
}
