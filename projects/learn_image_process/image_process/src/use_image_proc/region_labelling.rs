/**
 * https://docs.rs/imageproc/0.25.0/imageproc/region_labelling/index.html
 */
pub fn run() {
    log::debug!("region_labelling connected_components");
    let img = image::open("lena.png").expect("failed to load image");
    let image_gray = img.to_luma8();
    let conn = imageproc::region_labelling::Connectivity::Four;
    let background = image::Luma([0u8]);
    let result = imageproc::region_labelling::connected_components(&image_gray, conn, background);
    parse_to_lumau8(&result)
        .save("./results/region_labelling_connected_components.png")
        .unwrap();
}

fn parse_to_lumau8<P: num_traits::Num + image::Primitive + Into<f64>>(
    img: &imageproc::definitions::Image<image::Luma<P>>,
) -> imageproc::definitions::Image<image::Luma<u8>> {
    let parse_to_f64 = |value: P| -> f64 { value.into() };
    let (width, height) = img.dimensions();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|pixel| parse_to_f64(pixel.0[0]))
        .collect::<Vec<f64>>();
    let min_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let max_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let limit = min_value.abs().max(max_value.abs());
    let pixels = pixels
        .iter()
        .map(|pixel| {
            let pixel = *pixel;
            let sign = if pixel < 0f64 { -1f64 } else { 1f64 };
            let pixel = pixel * sign;
            128f64 * (pixel / limit) * sign
        })
        .collect::<Vec<f64>>();
    let correction_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let correction_value = if correction_value < 0.0f64 {
        correction_value.abs()
    } else {
        0f64
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
