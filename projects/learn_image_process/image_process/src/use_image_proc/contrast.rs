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
    log::debug!("contrast equalize_histogram");
    // ImageResult<DynamicImage>
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::contrast::equalize_histogram(&img_gray);
    result
        .save("./results/contrast_equalize_histogram.png")
        .expect("failed to save equalize_histogram image");
}

fn equalize_histogram_mut() {
    log::debug!("contrast equalize_histogram_mut");
    // ImageResult<DynamicImage>
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    imageproc::contrast::equalize_histogram_mut(&mut img_gray);
    img_gray
        .save("./results/contrast_equalize_histogram_mut.png")
        .expect("failed to save equalize_histogram_mut image");
}

fn match_histogram() {
    log::debug!("contrast match_histogram");
    // ImageResult<DynamicImage>
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let img_gradation = image::open("gradation.png").expect("failed to load image");
    let img_gradation_gray = img_gradation.to_luma8();
    let result = imageproc::contrast::match_histogram(&img_gray, &img_gradation_gray);
    result
        .save("./results/contrast_match_histogram.png")
        .expect("failed to save match_histogram image");
}

fn match_histogram_mut() {
    log::debug!("contrast match_histogram_mut");
    // ImageResult<DynamicImage>
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    let img_gradation = image::open("gradation.png").expect("failed to load image");
    let img_gradation_gray = img_gradation.to_luma8();
    imageproc::contrast::match_histogram_mut(&mut img_gray, &img_gradation_gray);
    img_gray
        .save("./results/contrast_match_histogram_mut.png")
        .expect("failed to save match_histogram_mut image");
}

fn otsu_level() {
    log::debug!("contrast otsu_level");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::contrast::otsu_level(&img_gray);
    log::info!("Otsu threshold level: {}", result);
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
