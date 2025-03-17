use image::GenericImageView;

pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    let image_buffer = img.clone().into_rgba32f();
    let (width, height) = img.dimensions();
    let center_x = width as f32 * 0.5f32;
    let center_y = height as f32 * 0.5f32;
    let theta = 45.0f32;
    // https://docs.rs/imageproc/0.25.0/imageproc/geometric_transformations/enum.Interpolation.html
    let interpolation = imageproc::geometric_transformations::Interpolation::Nearest;
    let default = image::Rgba([0f32, 0f32, 0f32, 0f32]);
    let result = imageproc::geometric_transformations::rotate(
        &image_buffer,
        (center_x, center_y),
        theta,
        interpolation,
        default,
    );
    let result = image::DynamicImage::ImageRgba32F(result);
    result 
        .into_rgba8()
        .save("geometric_transformation_rotate.png")
        .expect("failed to save geometric_transformation_rotate image");
    /*
    imageproc::geometric_transformation::rotate_about_center
    imageproc::geometric_transformation::translate
    imageproc::geometric_transformation::warp
    imageproc::geometric_transformation::warp_into
    imageproc::geometric_transformation::warp_into_with
    imageproc::geometric_transformation::warp_with
     */
}
