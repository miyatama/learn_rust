pub fn run() {
    corners_fast9();
    corners_fast12();
    fast_corner_score();
    oriented_fast();
}

fn corners_fast9() {
    log::debug!("corners corners_fast9");
    let mut img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();

    let result = imageproc::corners::corners_fast9(&img_gray, 100);
    // https://docs.rs/imageproc/0.25.0/imageproc/corners/struct.Corner.html

    let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    result.iter().for_each(|corner| {
        imageproc::drawing::draw_hollow_circle_mut(
            &mut img,
            (corner.x as i32, corner.y as i32),
            10i32,
            red,
        );
    });

    img.save("./results/corners_corners_fast9.png").unwrap();
}

fn corners_fast12() {
    log::debug!("corners corners_fast12");
}

fn fast_corner_score() {
    log::debug!("corners fast_corner_score");
}

fn oriented_fast() {
    log::debug!("corners oriented_fast");
}
