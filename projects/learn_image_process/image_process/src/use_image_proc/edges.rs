pub fn run() {
    canny();
}

fn canny() {
    log::debug!("edges canny");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let result = imageproc::edges::canny(&img_gray, 30f32, 240f32);
    result.save("./results/edges_canny.png").unwrap();
}
