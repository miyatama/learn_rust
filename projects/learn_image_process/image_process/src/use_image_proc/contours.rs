pub fn run() {
    log::info!("imageproc contours module");

    // TODO 元画像差し替え
    let mut img = image::open("lena.png").expect("failed to load image");
    // let mut img = image::open("contour_base.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    log::info!("call find_contours");
    let result = imageproc::contours::find_contours::<u8>(&img_gray);
    log::info!("called find_contours");
    let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    result.iter().for_each(|contour| {
        log::debug!("{:?}", &contour);
    });

    // img.save("contours_find_contours.png").unwrap();
    // imageproc::contours::find_contours_with_threshold
}
