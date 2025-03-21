use std::u8;

use image::GenericImageView;

pub fn run() {
    horizontal_prewitt();
    horizontal_scharr();
    horizontal_sobel();
    prewitt_gradients();
    sobel_gradient_map();
    sobel_gradients();
    vertical_prewitt();
    vertical_scharr();
    vertical_sobel();
}

fn horizontal_prewitt() {
    log::debug!("gradients horizontal_prewitt");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::horizontal_prewitt(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_horizontal_prewitt.png")
        .unwrap();
}

fn horizontal_scharr() {
    log::debug!("gradients horizontal_scharr");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::horizontal_scharr(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_horizontal_scharr.png")
        .unwrap();
}

fn horizontal_sobel() {
    log::debug!("gradients horizontal_sobel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::horizontal_sobel(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_horizontal_sobel.png")
        .unwrap();
}

fn prewitt_gradients() {
    log::debug!("gradients prewitt_gradients");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::prewitt_gradients(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_prewitt_gradients.png")
        .unwrap();
}

fn sobel_gradient_map() {
    log::debug!("gradients sobel_gradient_map");
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let img = img.to_rgb8();
    let result = imageproc::gradients::sobel_gradient_map(&img, |rgb| rgb);
    let max_value = result
        .clone()
        .pixels()
        .into_iter()
        .map(|pixel| pixel.0)
        .flatten()
        .max()
        .unwrap() as f32;
    let rgbs = result
        .clone()
        .pixels()
        .into_iter()
        .map(|pixel| {
            let r = (pixel.0[0] as f32 / max_value * u8::MAX as f32) as u8;
            let g = (pixel.0[1] as f32 / max_value * u8::MAX as f32) as u8;
            let b = (pixel.0[2] as f32 / max_value * u8::MAX as f32) as u8;
            [r, g, b]
        })
        .flatten()
        .collect::<Vec<u8>>();
    image::RgbImage::from_raw(width, height, rgbs)
        .unwrap()
        .save("./results/gradients_sobel_gradient_map.png")
        .unwrap();
}

fn sobel_gradients() {
    log::debug!("gradients sobel_gradients");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::sobel_gradients(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_sobel_gradients.png")
        .unwrap();
}

fn vertical_prewitt() {
    log::debug!("gradients vertical_prewitt");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::vertical_prewitt(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_vertical_prewitt.png")
        .unwrap();
}

fn vertical_scharr() {
    log::debug!("gradients vertical_scharr");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::vertical_scharr(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_vertical_scharr.png")
        .unwrap();
}

fn vertical_sobel() {
    log::debug!("gradients vertical_sobel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::vertical_sobel(&img_gray);
    let result = parse_to_lumau8(&result);
    result
        .save("./results/gradients_vertical_sobel.png")
        .unwrap();
}

fn parse_to_lumau8<P: num_traits::Num + image::Primitive + Into<f32>>(
    img: &imageproc::definitions::Image<image::Luma<P>>,
) -> imageproc::definitions::Image<image::Luma<u8>> {
    let parse_to_f32 = |value: P| -> f32 { value.into() };
    let (width, height) = img.dimensions();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|pixel| parse_to_f32(pixel.0[0]))
        .collect::<Vec<f32>>();
    let min_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let max_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let limit = min_value.abs().max(max_value.abs());
    let pixels = pixels
        .iter()
        .map(|pixel| {
            let pixel = *pixel;
            let sign = if pixel < 0f32 { -1f32 } else { 1f32 };
            let pixel = pixel * sign;
            128f32 * (pixel / limit) * sign
        })
        .collect::<Vec<f32>>();
    let correction_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let correction_value = if correction_value < 0.0f32 {
        correction_value.abs()
    } else {
        0f32
    };
    let pixels = pixels
        .iter()
        .map(|value| {
            let value = *value;
            (value + correction_value) as u8
        })
        .collect::<Vec<u8>>();
    image::GrayImage::from_raw(width, height, pixels).unwrap()
}
