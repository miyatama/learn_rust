use image::GenericImageView;

/**
 * https://docs.rs/imageproc/0.25.0/imageproc/haar/index.html
 */
pub fn run() {
    draw_haar_feature();
    draw_haar_feature_mut();
    enumerate_haar_features();
    number_of_haar_features();
}

fn draw_haar_feature() {
    log::debug!("haar draw_haar_feature");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let (width, height) = img.dimensions();
    let features = imageproc::haar::enumerate_haar_features(width as u8, height as u8);
    let result = imageproc::haar::draw_haar_feature(&img_gray, features[0]);
    result.save("./results/haar_draw_haar_feature.png").unwrap();
}

fn draw_haar_feature_mut() {
    log::debug!("haar draw_haar_feature_mut");
    /*
    let mut img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let features = imageproc::haar::enumerate_haar_features(width as u8, height as u8);
    features[0..10].iter().for_each(|feature| {
        imageproc::haar::draw_haar_feature_mut(&mut img, *feature);
    });
    img.save("./results/haar_draw_haar_feature_mut.png")
        .unwrap();
     */
}

/**
 * https://docs.rs/imageproc/0.25.0/imageproc/haar/fn.enumerate_haar_features.html
 */
fn enumerate_haar_features() {
    log::debug!("haar enumerate_haar_features");
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let features = imageproc::haar::enumerate_haar_features(width as u8, height as u8);
    log::info!("features size : {:?}", features.len());
    let img_gray = parse_to_lumau32(&img.to_luma32f());
    let threshold = 20i32;
    let features = features
        .iter()
        .filter(|feature| {
            let evaluate_result = feature.evaluate(&img_gray);
            log::info!(
                "feature: {:?}, evaluate reult: {}",
                feature,
                evaluate_result
            );
            evaluate_result > threshold
        })
        .collect::<Vec<_>>();
    log::info!(
        "length of {} over feature is {}",
        threshold,
        features.len()
    );
}

fn number_of_haar_features() {
    log::debug!("haar number_of_haar_features");
    let result = imageproc::haar::number_of_haar_features(100u32, 100u32);
    log::info!("number_of_haar_features result: {}", result);
}

fn parse_to_lumau32<P: num_traits::Num + image::Primitive + Into<f32>>(
    img: &imageproc::definitions::Image<image::Luma<P>>,
) -> imageproc::definitions::Image<image::Luma<u32>> {
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
            (u32::MAX / 2) as f32 * (pixel / limit) * sign
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
            (value + correction_value) as u32
        })
        .collect::<Vec<u32>>();
    image::ImageBuffer::<image::Luma<u32>, Vec<u32>>::from_raw(width, height, pixels).unwrap()
}
