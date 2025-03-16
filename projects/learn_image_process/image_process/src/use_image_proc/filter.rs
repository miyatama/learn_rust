use image::GenericImageView;

/**
 * filterの呼び出しサンプル
 * https://docs.rs/imageproc/0.25.0/imageproc/filter/index.html
 */
pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    log::debug!("image type: {}", get_type_name(img.clone()));

    let (width, height) = img.dimensions();
    let img_gray = img.to_luma8();
    let img_result = imageproc::filter::bilateral_filter(&img_gray, 10, 0.5, 0.2);
    // ImageBuffer<image::color::Luma<u8>, alloc::vec::Vec<u8>>
    log::debug!(
        "bilateral_filter result type: {}",
        get_type_name(img_result.clone())
    );
    img_result
        .save("filter_bilateral_filter.png")
        .expect("failed to save bilateral_filter image");

    let img_result = imageproc::filter::box_filter(&img_gray, 10, 10);
    img_result
        .save("filter_box_filter.png")
        .expect("failed to save box_filter image");

    let kernel = [
        1.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        1.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        4.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        1.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        1.0f32 / 16.0f32,
    ];
    let image_buffer = img.clone().into_rgb32f();
    // type: image::buffer_::ImageBuffer<image::color::Rgb<f32>, alloc::vec::Vec<f32>>
    let filter_result =
        imageproc::filter::filter3x3::<image::Rgb<f32>, f32, f32>(&image_buffer, &kernel);
    let filter_result = image::DynamicImage::ImageRgb32F(filter_result);
    filter_result
        .into_rgb8()
        .save("filter_filter3x3.png")
        .expect("failed to save filter3x3 image");

    let filter_result = imageproc::filter::gaussian_blur_f32(&image_buffer, 10.5f32);
    let filter_result = image::DynamicImage::ImageRgb32F(filter_result);
    filter_result
        .into_rgb8()
        .save("filter_gaussian_blur_f32.png")
        .expect("failed to save gaussian_blur_f32 image");

    let filter_result = imageproc::filter::horizontal_filter(&image_buffer, &kernel);
    let filter_result = image::DynamicImage::ImageRgb32F(filter_result);
    filter_result
        .into_rgb8()
        .save("filter_horizontal_filter.png")
        .expect("failed to save horizontal_filter image");

    // pub fn laplacian_filter(image: &GrayImage) -> Image<Luma<i16>>
    // pub type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;
    // pub struct ImageBuffer<P: Pixel, Container>
    // ImageBuffer<Luma<i16>, Vec<i16>>
    let laplacian_filter_result = imageproc::filter::laplacian_filter(&img_gray);
    // ImageBuffer<image::color::Luma<i16>, alloc::vec::Vec<i16>>
    log::debug!(
        "laplacian_filter result type: {}",
        get_type_name(laplacian_filter_result.clone())
    );
    let pixels = laplacian_filter_result
        .pixels()
        .into_iter()
        .map(|pixel| pixel.0[0] as f32)
        .collect::<Vec<f32>>();
    let min_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let max_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let limit = min_value.abs().max(max_value.abs());
    let pixels = pixels
        .iter()
        .map(|pixel| {
            let pixel = *pixel;
            let sign = if pixel < 0f32 { -1f32 } else { 1f32 };
            let pixel = pixel * sign;
            128f32 * (pixel / limit) * sign
        })
        .collect::<Vec<f32>>();
    let correction_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let correction_value = if correction_value < 0.0f32 {
        correction_value.abs()
    } else {
        0f32
    };
    let pixels = pixels
        .iter()
        .map(|value| {
            let value = *value;
            (value + correction_value) as u8
        })
        .collect::<Vec<u8>>();
    image::GrayImage::from_raw(width, height, pixels)
        .unwrap()
        .save("filter_laplacian_filter.png")
        .expect("failed to save laplacian_filter image");

    let image_buffer = img.clone().into_rgb8();
    let filter_result = imageproc::filter::median_filter(&image_buffer, 3u32, 3u32);
    filter_result
        .save("filter_median_filter.png")
        .expect("failed to save median_filter image");

    let h_kernel = [1.0f32 / 4.0f32, 2.0f32 / 4.0f32, 1.0f32 / 4.0f32];
    let v_kernel = [1.0f32 / 5.0f32, 3.0f32 / 5.0f32, 1.0f32 / 5.0f32];
    let image_buffer = img.clone().into_rgb32f();
    let filter_result = imageproc::filter::separable_filter::<image::Rgb<f32>, f32>(
        &image_buffer,
        &h_kernel,
        &v_kernel,
    );
    let filter_result = image::DynamicImage::ImageRgb32F(filter_result);
    filter_result
        .into_rgb8()
        .save("separable_filter.png")
        .expect("failed to save separable_filter image");

    /*
    let img_result = imageproc::filter::separable_filter_equal

    let img_result = imageproc::filter::sharpen3x3

    let img_result = imageproc::filter::sharpen_gaussian

    let img_result = imageproc::filter::vertical_filter

             */
}

#[allow(dead_code)]
fn get_type_name<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}
