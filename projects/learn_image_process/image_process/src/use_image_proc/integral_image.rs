/**
 * https://docs.rs/imageproc/0.25.0/imageproc/integral_image/index.html
 */
pub fn run() {
    column_running_sum();
    integral_image();
    integral_squared_image();
    row_running_sum();
    sum_image_pixels();
    variance();
}

fn column_running_sum() {
    log::debug!("integral_image column_running_sum");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let (width, height) = img_gray.dimensions();
    let padding = 1 as u32;
    // let mut buffer = [0; height + 2]; <- non const value
    let mut buffer = vec![0; height as usize + 2usize];
    let column = width - padding;

    imageproc::integral_image::column_running_sum(&img_gray, column, &mut buffer, padding);
    log::info!("column_running_sum result: {:?}", buffer);
}

fn integral_image() {
    log::debug!("integral_image integral_image");
}

fn integral_squared_image() {
    log::debug!("integral_image integral_squared_image");
}

fn row_running_sum() {
    log::debug!("integral_image row_running_sum");
}

fn sum_image_pixels() {
    log::debug!("integral_image sum_image_pixels");
}

fn variance() {
    log::debug!("integral_image variance");
}
