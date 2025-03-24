/**
 * https://docs.rs/imageproc/0.25.0/imageproc/hough/index.html
 */
pub fn run() {
    log::debug!("hough detect_lines");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let options = imageproc::hough::LineDetectionOptions {
        vote_threshold: 50u32,
        suppression_radius: 5u32,
    };
    let result = imageproc::hough::detect_lines(&img_gray, options);

    let mut image_buffer = img.to_rgb8();
    let red = image::Rgb([255u8, 0u8, 0u8]);
    log::debug!("hough draw_polar_lines_mut");
    imageproc::hough::draw_polar_lines_mut(&mut image_buffer, &result, red);
    image_buffer
        .save("./results/hough_draw_polar_lines_mut.png")
        .unwrap();

    log::debug!("hough draw_polar_lines");
    let image_buffer = img.to_rgb8();
    let result = imageproc::hough::draw_polar_lines(&image_buffer, &result, red);
    result.save("./results/hough_draw_polar_lines.png").unwrap();
}
