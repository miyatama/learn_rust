pub fn run() {
    distance_transform();
    distance_transform_mut();
    euclidean_squared_distance_transform();
}

fn distance_transform() {
    log::debug!("distance_transform");
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.clone().to_luma8();
    imageproc::contrast::threshold_mut(
        &mut img_gray,
        100u8,
        imageproc::contrast::ThresholdType::Binary,
    );
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/enum.Norm.html
    let norm = imageproc::distance_transform::Norm::L1;
    // https://docs.rs/imageproc/0.25.0/imageproc/distance_transform/fn.distance_transform.html
    let result = imageproc::distance_transform::distance_transform(&img_gray, norm);
    result 
        .save("./results/distance_transform_distance_transform.png")
        .unwrap();
}

fn distance_transform_mut() {
    log::debug!("distance_transform_mut");
}

fn euclidean_squared_distance_transform() {
    log::debug!("euclidean_squared_distance_transform");
}
