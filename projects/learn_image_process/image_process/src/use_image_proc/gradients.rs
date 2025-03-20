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
    let result = parse_lumai16_to_lumau8(&result);
    result
        .save("./results/gradients_horizontal_prewitt.png")
        .unwrap();
}

fn horizontal_scharr() {
    log::debug!("gradients horizontal_scharr");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();

    let result = imageproc::gradients::horizontal_scharr(&img_gray);
    let result = parse_lumai16_to_lumau8(&result);
    result
        .save("./results/gradients_horizontal_scharr.png")
        .unwrap();
}

fn horizontal_sobel() {
    log::debug!("gradients horizontal_sobel");
}

fn prewitt_gradients() {
    log::debug!("gradients prewitt_gradients");
}

fn sobel_gradient_map() {
    log::debug!("gradients sobel_gradient_map");
}

fn sobel_gradients() {
    log::debug!("gradients sobel_gradients");
}

fn vertical_prewitt() {
    log::debug!("gradients vertical_prewitt");
}

fn vertical_scharr() {
    log::debug!("gradients vertical_scharr");
}

fn vertical_sobel() {
    log::debug!("gradients vertical_sobel");
}

fn parse_lumai16_to_lumau8(
    img: &imageproc::definitions::Image<image::Luma<i16>>,
) -> imageproc::definitions::Image<image::Luma<u8>> {
    let (width, height) = img.dimensions();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|pixel| pixel.0[0] as f32)
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
