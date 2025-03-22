use image::GenericImageView;

/**
 * https://docs.rs/imageproc/0.25.0/imageproc/haar/index.html
 */
pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let image_buffer = img.to_rgba32f();
    let scale = 50.0 / width.max(height) as f32;
    log::debug!("haar resize scale: {}", scale);
    let projection = imageproc::geometric_transformations::Projection::scale(scale, scale);
    let interpolation = imageproc::geometric_transformations::Interpolation::Nearest;
    let default = image::Rgba([0f32, 0f32, 0f32, 0f32]);
    let (new_width, new_height) = (
        (scale * width as f32) as u32,
        (scale * height as f32) as u32,
    );
    let mut resized_image_buffer = image::Rgba32FImage::new(new_width, new_height);
    imageproc::geometric_transformations::warp_into(
        &image_buffer,
        &projection,
        interpolation,
        default,
        &mut resized_image_buffer,
    );
    let (width, height) = resized_image_buffer.dimensions();
    log::debug!("haar scaled size: ({}, {})", width, height);

    log::debug!("haar number_of_haar_features");
    let result = imageproc::haar::number_of_haar_features(width, height);
    log::info!("number_of_haar_features result: {}", result);

    log::debug!("haar enumerate_haar_features");
    // https://docs.rs/imageproc/0.25.0/imageproc/haar/fn.enumerate_haar_features.html
    let features = imageproc::haar::enumerate_haar_features(width as u8, height as u8);
    log::info!("features size : {:?}", features.len());

    let img = image::DynamicImage::ImageRgba32F(resized_image_buffer);
    let mut gray_image = img.clone().to_luma8();
    let result = imageproc::haar::draw_haar_feature(&gray_image, features[0]);
    result.save("./results/haar_draw_haar_feature.png").unwrap();

    features[0..10].iter().for_each(|feature| {
        imageproc::haar::draw_haar_feature_mut(&mut gray_image, *feature);
    });
    gray_image
        .save("./results/haar_draw_haar_feature_mut.png")
        .unwrap();
}
