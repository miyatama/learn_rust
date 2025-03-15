/**
 * filterの呼び出しサンプル
 * https://docs.rs/imageproc/0.25.0/imageproc/filter/index.html
 */
pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let img_result = imageproc::filter::bilateral_filter(&img_gray, 10, 0.5, 0.2);
    img_result
        .save("bilateral_filter_result.png")
        .expect("failed to save bilateral_filter image");
    let img_result = imageproc::filter::box_filter(&img_gray, 10, 10);
    img_result
        .save("box_filter_result.png")
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
    let image_buffer = img.into_rgb32f();
    // type: image::buffer_::ImageBuffer<image::color::Rgb<f32>, alloc::vec::Vec<f32>>
    let filter_result =
        imageproc::filter::filter3x3::<image::Rgb<f32>, f32, f32>(&image_buffer, &kernel);
    log::debug!(
        "filter result type: {}",
        get_type_name(filter_result.clone())
    );
    let filter_result = image::DynamicImage::ImageRgb32F(filter_result);
    log::debug!(
        "parsed filter result type: {}",
        get_type_name(filter_result.clone())
    );
    filter_result
        .into_rgb8()
        .save("filter3x3_result.png")
        .expect("failed to save filter3x3 image");

    /*
    let img_result = gaussian_blur_f32

    let img_result = horizontal_filter

    let img_result = laplacian_filter

    let img_result = median_filter

    let img_result = separable_filter

    let img_result = separable_filter_equal

    let img_result = sharpen3x3

    let img_result = sharpen_gaussian

    let img_result = vertical_filter

             */
}

fn get_type_name<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}
