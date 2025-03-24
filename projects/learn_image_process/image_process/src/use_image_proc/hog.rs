/**
 * https://docs.rs/imageproc/0.25.0/imageproc/hog/index.html
 */
use image::GenericImageView;

pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();

    let image_buffer = img.to_rgba32f();
    let (scale_x, scale_y) = (400f32 / width as f32, 400f32 / height as f32);
    let projection = imageproc::geometric_transformations::Projection::scale(scale_x, scale_y);
    let interpolation = imageproc::geometric_transformations::Interpolation::Nearest;
    let default = image::Rgba([0f32, 0f32, 0f32, 0f32]);
    let (new_width, new_height) = (
        (scale_x * width as f32) as u32,
        (scale_y * height as f32) as u32,
    );
    let mut resized_image_buffer = image::Rgba32FImage::new(new_width, new_height);
    imageproc::geometric_transformations::warp_into(
        &image_buffer,
        &projection,
        interpolation,
        default,
        &mut resized_image_buffer,
    );
    let (width, height) = resized_image_buffer.dimensions();
    log::debug!("hog width, height: ({}, {})", width, height);
    let img = image::DynamicImage::ImageRgba32F(resized_image_buffer);
    let img_gray = img.clone().to_luma8();
    let orientations = 15;
    let signed = false;
    let cell_side = 100usize; // evenly divide width & height
    let block_side = 2;
    // (width, height) = (400, 400)
    let cell_wide = width as usize / cell_side;
    let cell_height = height as usize / cell_side;
    log::debug!(
        "(cell_wide, cell_height) = ({}, {})",
        cell_wide,
        cell_height
    );
    let block_stride = 2; // evenly divide (cells high(= height / cell_side) - block side)
    let options =
        imageproc::hog::HogOptions::new(orientations, signed, cell_side, block_side, block_stride);
    let spec = imageproc::hog::HogSpec::from_options(width, height, options);
    match spec {
        Ok(spec) => {
            log::debug!("hog cell_histograms");
            let mut array_3d = imageproc::hog::cell_histograms(&img_gray, spec);
            let view = array_3d.view_mut();
            log::debug!("hog render_hist_grid");
            let result = imageproc::hog::render_hist_grid(10u32, &view, signed);
            result.save("./results/hog_render_hist_grid.png").unwrap();
        }
        Err(e) => {
            log::error!("HogSpec::from_options error: {:?}", e);
        }
    }
    log::debug!("called hog");
    match imageproc::hog::hog(&img_gray, options) {
        Ok(vectors) => {
            log::info!("hog result: {:?}", vectors);
        }
        Err(e) => {
            log::error!("imageproc::hog::hog error: {:?}", e);
        }
    }
}
