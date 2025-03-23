/**
 * https://docs.rs/imageproc/0.25.0/imageproc/hog/index.html
 */
use image::GenericImageView;

pub fn run() {
    log::debug!("hog cell_histograms");
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let img_gray = img.clone().to_luma8();
    let options = imageproc::hog::HogOptions::new(
        15, false, 5, // evenly divide height 225
        10, 5, // evenly divide (cells high - block side)
    );
    let spec = imageproc::hog::HogSpec::from_options(width, height, options);
    match spec {
        Ok(spec) => {
            let mut array_3d = imageproc::hog::cell_histograms(&img_gray, spec);
            let view = array_3d.view_mut();
            log::info!("called cell_histograms");
            let result = imageproc::hog::render_hist_grid(10u32, &view, false);
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
