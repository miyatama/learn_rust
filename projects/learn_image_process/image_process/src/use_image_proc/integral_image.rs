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
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let result = imageproc::integral_image::integral_image::<_, u32>(&img_gray);
    parse_to_lumau8(&result)
        .save("./results/integral_image_integral_image.png")
        .unwrap();
}

fn integral_squared_image() {
    log::debug!("integral_image integral_squared_image");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let result = imageproc::integral_image::integral_squared_image::<_, u32>(&img_gray);
    parse_to_lumau8(&result)
        .save("./results/integral_image_integral_squared_image.png")
        .unwrap();
}

fn row_running_sum() {
    log::debug!("integral_image row_running_sum");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let (width, height) = img_gray.dimensions();
    let padding = 1 as u32;
    // let mut buffer = [0; height + 2]; <- non const value
    let mut buffer = vec![0; width as usize + 2usize];
    let row = height - padding;

    imageproc::integral_image::row_running_sum(&img_gray, row, &mut buffer, padding);
    log::info!("row_running_sum result: {:?}", buffer);
}

fn sum_image_pixels() {
    log::debug!("integral_image sum_image_pixels");
    let img = image::open("lena.png").expect("failed to load image");
    let image_buffer = img.to_rgb8();
    let (width, height) = image_buffer.dimensions();
    let result = imageproc::integral_image::sum_image_pixels(
        &image_buffer,
        0u32,
        0u32,
        width - 2,
        height - 2,
    );
    log::info!("sum_image_pixels result: {:?}", result);
}

fn variance() {
    log::debug!("integral_image variance");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let (width, height) = img_gray.dimensions();
    let integral_image = imageproc::integral_image::integral_image::<_, u32>(&img_gray);
    let integral_squared_image =
        imageproc::integral_image::integral_squared_image::<_, u32>(&img_gray);
    let result = imageproc::integral_image::variance(
        &integral_image,
        &integral_squared_image,
        0u32,
        0u32,
        width - 2,
        height - 2,
    );
    log::info!("variance result: {:?}", result);
}

fn parse_to_lumau8<P: num_traits::Num + image::Primitive + Into<f64>>(
    img: &imageproc::definitions::Image<image::Luma<P>>,
) -> imageproc::definitions::Image<image::Luma<u8>> {
    let parse_to_f64 = |value: P| -> f64 { value.into() };
    let (width, height) = img.dimensions();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|pixel| parse_to_f64(pixel.0[0]))
        .collect::<Vec<f64>>();
    let min_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let max_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let limit = min_value.abs().max(max_value.abs());
    let pixels = pixels
        .iter()
        .map(|pixel| {
            let pixel = *pixel;
            let sign = if pixel < 0f64 { -1f64 } else { 1f64 };
            let pixel = pixel * sign;
            128f64 * (pixel / limit) * sign
        })
        .collect::<Vec<f64>>();
    let correction_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let correction_value = if correction_value < 0.0f64 {
        correction_value.abs()
    } else {
        0f64
    };
    let pixels = pixels
        .iter()
        .map(|value| {
            let value = *value;
            (value + correction_value) as u8
        })
        .collect::<Vec<u8>>();
    image::GrayImage::from_raw(width, height, pixels).unwrap()
}
