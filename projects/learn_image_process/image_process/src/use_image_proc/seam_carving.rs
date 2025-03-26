pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    let image_gray = img.clone().to_luma8();
    log::debug!("seam_carving find_vertical_seam");
    let seam = imageproc::seam_carving::find_vertical_seam(&image_gray);

    log::debug!("seam_carving draw_vertical_seams");
    let seams = vec![seam];
    let image_result = imageproc::seam_carving::draw_vertical_seams(&image_gray, &seams);
    image_result
        .save("./results/seam_carving_draw_vertical_seams.png")
        .unwrap();

    log::debug!("seam_carving remove_vertical_seam");
    let seam = imageproc::seam_carving::find_vertical_seam(&image_gray);
    let image_result = imageproc::seam_carving::remove_vertical_seam(&image_gray, &seam);
    image_result
        .save("./results/seam_carving_remove_vertical_seam.png")
        .unwrap();

    log::debug!("seam_carving shrink_width(skip)");
    /*
    let image_result = imageproc::seam_carving::shrink_width(&image_gray, 100u32);
    image_result
        .save("./results/seam_carving_shrink_width.png")
        .unwrap();
     */
}
