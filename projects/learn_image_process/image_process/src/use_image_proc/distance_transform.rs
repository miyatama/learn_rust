use image::GenericImageView;

pub fn run() {
    distance_transform();
    distance_transform_mut();
    euclidean_squared_distance_transform();
}

fn distance_transform() {
    log::debug!("distance_transform");
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.clone().to_luma8();
    imageproc::contrast::threshold_mut(
        &mut img_gray,
        100u8,
        imageproc::contrast::ThresholdType::Binary,
    );
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/enum.Norm.html
    let norm = imageproc::distance_transform::Norm::L1;
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/fn.distance_transform.html
    let result = imageproc::distance_transform::distance_transform(&img_gray, norm);
    result
        .save("./results/distance_transform_distance_transform.png")
        .unwrap();
}

fn distance_transform_mut() {
    log::debug!("distance_transform_mut");
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.clone().to_luma8();
    imageproc::contrast::threshold_mut(
        &mut img_gray,
        100u8,
        imageproc::contrast::ThresholdType::Binary,
    );
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/enum.Norm.html
    let norm = imageproc::distance_transform::Norm::L1;
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/fn.distance_transform.html
    imageproc::distance_transform::distance_transform_mut(&mut img_gray, norm);
    img_gray
        .save("./results/distance_transform_distance_transform_mut.png")
        .unwrap();
}

fn euclidean_squared_distance_transform() {
    log::debug!("euclidean_squared_distance_transform");
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let mut img_gray = img.clone().to_luma8();
    imageproc::contrast::threshold_mut(
        &mut img_gray,
        100u8,
        imageproc::contrast::ThresholdType::Binary,
    );
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/enum.Norm.html
    let result = imageproc::distance_transform::euclidean_squared_distance_transform(&img_gray);
    let pixels = result
        .pixels()
        .into_iter()
        .map(|pixel| pixel.0[0])
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
            127f64 * (pixel / limit) * sign
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
    image::GrayImage::from_raw(width, height, pixels)
        .unwrap()
        .save("./results/distance_transform_euclidean_squared_distance_transform.png")
        .expect("failed to save euclidean_squared_distance_transform image");
}
