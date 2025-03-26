pub fn run() {
    local_maxima();
    suppress_non_maximum();
}

fn local_maxima() {
    log::debug!("suppress local_maxima");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let corners = imageproc::corners::corners_fast9(&img_gray, 100);
    let radius = 50u32;
    let result = imageproc::suppress::local_maxima(&corners, radius);
    log::info!("local_maxima result: {:?}", result);
}
fn suppress_non_maximum() {
    log::debug!("suppress suppress_non_maximum");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let radius = 50u32;
    let result = imageproc::suppress::suppress_non_maximum(&img_gray, radius);
    result
        .save("./results/suppress_suppress_non_maximum.png")
        .unwrap();
}
