use crate::util::custom_error::CustomError;
use image::{DynamicImage, GenericImageView, Rgba};
use std::collections::HashMap;

pub fn apply(
    img: DynamicImage,
    degree: i32,
) -> Result<Result<DynamicImage, CustomError>, anyhow::Error> {
    log::debug!("rotate::apply");
    let rad = degree as f64 * std::f64::consts::PI / 180f64;
    let rotation = |x: f64, y: f64| -> (f64, f64) {
        (x * rad.cos() - y * rad.sin(), x * rad.sin() + y * rad.cos())
    };
    let (width, height) = img.dimensions();
    log::debug!("before rotated rect((0, 0), ({}, {}))", width, height);
    let rotated_rect = vec![
        (0f64, 0f64),
        (0f64, height as f64),
        (width as f64, 0f64),
        (width as f64, height as f64),
    ]
    .iter()
    .map(|(x, y)| rotation(*x, *y))
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

    log::debug!(
        "after rotated rect(({}, {}), ({}, {}))",
        minx,
        miny,
        maxx,
        maxy
    );

    let new_width = (maxx - minx).trunc() as u32;
    let new_height = (maxy - miny).trunc() as u32;

    log::debug!("new rect({}, {})", new_width, new_height);
    let x_pad = minx.trunc() * -1f64;
    let y_pad = miny.trunc() * -1f64;
    let mut result_pixels: Vec<Rgba<u8>> = Vec::with_capacity((new_width * new_height) as usize);
    let mut hm: HashMap<usize, Rgba<u8>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let (new_x, new_y) = rotation(x as f64, y as f64);
            let (new_x, new_y) = (new_x.trunc(), new_y.trunc());
            let pos = (new_x + x_pad + (new_y + y_pad) * new_width as f64) as usize;
            let pixel = img.get_pixel(x, y).0;
            // result_pixels[pos] = Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255])
            hm.insert(
                pos,
                Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255]),
            );
        }
    }
    for y in 0..new_height {
        for x in 0..new_width {
            let pos = (x + y * new_width) as usize;
            match hm.get(&pos) {
                Some(pixel) => {
                    result_pixels.push(*pixel);
                }
                None => {
                    // 上下左右に値がある場合は平均値を設定
                    let mut set_pixel = false;
                    if x > 0 && x < new_width && y > 0 && y < new_height {
                        let colors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                            .iter()
                            .map(|(x, y)| x + y * new_width)
                            .map(|pos| hm.get(&(pos as usize)))
                            .collect::<Vec<_>>();
                        let enable_color_count = colors
                            .iter()
                            .filter(|color| color.is_some())
                            .collect::<Vec<_>>()
                            .len() as u32;
                        if enable_color_count > 2 {
                            let mut r = 0u32;
                            let mut g = 0u32;
                            let mut b = 0u32;
                            colors
                                .iter()
                                .filter(|color| color.is_some())
                                .map(|color| color.unwrap())
                                .for_each(|rgb| {
                                    r += rgb[0] as u32;
                                    g += rgb[1] as u32;
                                    b += rgb[2] as u32;
                                });

                            result_pixels.push(Rgba([
                                (r / enable_color_count) as u8,
                                (g / enable_color_count) as u8,
                                (b / enable_color_count) as u8,
                                255u8,
                            ]));
                            set_pixel = true;
                        }
                    }
                    if !set_pixel {
                        result_pixels.push(Rgba([0u8, 0u8, 0u8, 0u8]));
                    }
                }
            }
        }
    }
    let result_pixels = result_pixels
        .into_iter()
        .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
        .flatten()
        .collect::<Vec<u8>>();
    let result_img = image::ImageBuffer::from_raw(new_width, new_height, result_pixels).unwrap();
    Ok(Ok(DynamicImage::ImageRgba8(result_img)))
}
