pub fn run() {
    close();
    close_mut();
    dilate();
    dilate_mut();
    erode();
    erode_mut();
    grayscale_close();
    grayscale_dilate();
    grayscale_erode();
    grayscale_open();
    open();
    open_mut();
}

fn close() {
    log::debug!("morphology close");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result =
        imageproc::morphology::close(&img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    result.save("./results/morphology_close.png").unwrap();
}

fn close_mut() {
    log::debug!("morphology close_mut");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    imageproc::morphology::close_mut(&mut img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    img_gray.save("./results/morphology_close_mut.png").unwrap();
}

fn dilate() {
    log::debug!("morphology dilate");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result =
        imageproc::morphology::dilate(&img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    result.save("./results/morphology_dilate.png").unwrap();
}

fn dilate_mut() {
    log::debug!("morphology dilate_mut");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    imageproc::morphology::dilate_mut(&mut img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    img_gray
        .save("./results/morphology_dilate_mut.png")
        .unwrap();
}

fn erode() {
    log::debug!("morphology erode");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result =
        imageproc::morphology::erode(&img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    result.save("./results/morphology_erode.png").unwrap();
}

fn erode_mut() {
    log::debug!("morphology erode_mut");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    imageproc::morphology::erode_mut(&mut img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    img_gray.save("./results/morphology_erode_mut.png").unwrap();
}

fn grayscale_close() {
    log::debug!("morphology grayscale_close");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::morphology::grayscale_close(
        &img_gray,
        &imageproc::morphology::Mask::square(15u8),
    );
    result
        .save("./results/morphology_grayscale_close.png")
        .unwrap();
}

fn grayscale_dilate() {
    log::debug!("morphology grayscale_dilate");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::morphology::grayscale_dilate(
        &img_gray,
        &imageproc::morphology::Mask::square(15u8),
    );
    result
        .save("./results/morphology_grayscale_dilate.png")
        .unwrap();
}

fn grayscale_erode() {
    log::debug!("morphology grayscale_erode");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::morphology::grayscale_erode(
        &img_gray,
        &imageproc::morphology::Mask::square(15u8),
    );
    result
        .save("./results/morphology_grayscale_erode.png")
        .unwrap();
}

fn grayscale_open() {
    log::debug!("morphology grayscale_open");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::morphology::grayscale_open(
        &img_gray,
        &imageproc::morphology::Mask::square(15u8),
    );
    result
        .save("./results/morphology_grayscale_open.png")
        .unwrap();
}

fn open() {
    log::debug!("morphology open");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result =
        imageproc::morphology::open(&img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    result.save("./results/morphology_open.png").unwrap();
}

fn open_mut() {
    log::debug!("morphology open_mut");
    let img = image::open("morphology_model.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    imageproc::morphology::open_mut(&mut img_gray, imageproc::distance_transform::Norm::L2, 20u8);
    img_gray.save("./results/morphology_open_mut.png").unwrap();
}
