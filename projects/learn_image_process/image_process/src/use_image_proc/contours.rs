/**
 * https://docs.rs/imageproc/0.25.0/imageproc/contours/index.html
 */
pub fn run() {
    log::info!("imageproc contours module");
    let img = image::open("contour_base.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let threshold = 100u8;
    let img_gray = imageproc::contrast::threshold(
        &img_gray,
        threshold,
        imageproc::contrast::ThresholdType::Binary,
    );
    img_gray
        .save("./results/contours_find_contours_threshold.png")
        .unwrap();
    log::info!("call find_contours");
    let result = imageproc::contours::find_contours::<u8>(&img_gray);
    log::info!("called find_contours");
    let _red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    result.iter().for_each(|contour| {
        log::debug!("{:?}", &contour);
    });
    img.save("./results/contours_find_contours.png").unwrap();

    let result = imageproc::contours::find_contours_with_threshold::<u8>(&img_gray, threshold);
    result.iter().for_each(|contour| {
        log::debug!("{:?}", &contour);
    });
    img.save("./results/contours_find_contours_with_threshold.png")
        .unwrap();
}
