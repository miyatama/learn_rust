/*
https://docs.rs/imageproc/0.25.0/imageproc/geometric_transformations/index.html
 */
use image::GenericImageView;

pub fn run() {
    // https://docs.rs/image/latest/image/fn.open.html
    // ImageResult<DynamicImage>
    let img = image::open("lena.png").expect("failed to load image");
    // https://docs.rs/image/latest/image/enum.DynamicImage.html#method.into_rgba32f
    // Rgba32FImage
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

    let result = imageproc::geometric_transformations::rotate_about_center(
        &image_buffer,
        theta,
        interpolation,
        default,
    );
    let result = image::DynamicImage::ImageRgba32F(result);
    result
        .into_rgba8()
        .save("geometric_transformation_rotate_about_center.png")
        .expect("failed to save rotate_about_center image");

    let result = imageproc::geometric_transformations::translate(
        &image_buffer,
        (center_x as i32, center_y as i32),
    );
    let result = image::DynamicImage::ImageRgba32F(result);
    result
        .into_rgba8()
        .save("geometric_transformation_translate.png")
        .expect("failed to save translate image");

    // https://docs.rs/imageproc/0.25.0/imageproc/geometric_transformations/struct.Projection.html
    let projection =
        imageproc::geometric_transformations::Projection::translate(center_x, center_y)
            * imageproc::geometric_transformations::Projection::scale(0.5f32, 0.5f32)
            * imageproc::geometric_transformations::Projection::rotate(45.0f32);
    let default = image::Rgba([0f32, 0f32, 0f32, 0f32]);
    let result = imageproc::geometric_transformations::warp(
        &image_buffer,
        &projection,
        interpolation,
        default,
    );
    let result = image::DynamicImage::ImageRgba32F(result);
    result
        .into_rgba8()
        .save("geometric_transformation_warp.png")
        .expect("failed to save warp image");

    let edge_length = width.max(height) * 2;
    // https://docs.rs/image/latest/image/type.Rgba32FImage.html#method.new
    let mut out_image = image::Rgba32FImage::new(edge_length, edge_length);
    imageproc::geometric_transformations::warp_into(
        &image_buffer,
        &projection,
        interpolation,
        default,
        &mut out_image,
    );
    let result = image::DynamicImage::ImageRgba32F(out_image);
    result
        .into_rgba8()
        .save("geometric_transformation_warp_into.png")
        .expect("failed to save warp_into image");
    /*
    imageproc::geometric_transformations::warp_into_with
     */
    let result = imageproc::geometric_transformations::warp_with(
        &image_buffer,
        |x, y| (x, y + (x / 30.0).sin() * 100.0),
        interpolation,
        default,
    );
    let result = image::DynamicImage::ImageRgba32F(result);
    result
        .into_rgba8()
        .save("geometric_transformation_warp_with.png")
        .expect("failed to save warp_with image");
}
