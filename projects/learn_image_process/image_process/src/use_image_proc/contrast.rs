pub fn run() {
    adaptive_threshold();
    equalize_histogram();
    equalize_histogram_mut();
    match_histogram();
    match_histogram_mut();
    otsu_level();
    stretch_contrast();
    stretch_contrast_mut();
    threshold();
    threshold_mut();
}

fn adaptive_threshold() {
    log::debug!("contrast adaptive_threshold");
    // ImageResult<DynamicImage>
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let block_radius = 20u32;
    // https://docs.rs/image/0.25.0/image/type.GrayImage.html
    let result = imageproc::contrast::adaptive_threshold(&img_gray, block_radius);
    result
        .save("./results/contrast_adaptive_threshold.png")
        .expect("failed to save adaptive_threshold image");
}
fn equalize_histogram() {
    // imageproc::contrast::equalize_histogram();
}
fn equalize_histogram_mut() {
    // imageproc::contrast::equalize_histogram_mut();
}
fn match_histogram() {
    // imageproc::contrast::match_histogram();
}
fn match_histogram_mut() {
    // imageproc::contrast::match_histogram_mut();
}
fn otsu_level() {
    // imageproc::contrast::otsu_level();
}
fn stretch_contrast() {
    // imageproc::contrast::stretch_contrast();
}
fn stretch_contrast_mut() {
    // imageproc::contrast::stretch_contrast_mut();
}
fn threshold() {
    // imageproc::contrast::threshold();
}
fn threshold_mut() {
    // imageproc::contrast::threshold_mut();
}
