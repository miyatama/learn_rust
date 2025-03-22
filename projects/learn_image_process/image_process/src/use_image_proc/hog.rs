use image::GenericImageView;

pub fn run() {
    cell_histograms();
    hog();
    render_hist_grid();
}

fn cell_histograms() {
    log::debug!("hog cell_histograms");
    let img = image::open("lena.png").expect("failed to load image");
    let (width, height) = img.dimensions();
    let img_gray = img.clone().to_luma8();
    let options = imageproc::hog::HogOptions::new(
        15,
        false,
        5,  // evenly divide height 225
        10,
        5,  // evenly divide (cells high - block side)
    );
    let spec = imageproc::hog::HogSpec::from_options(width, height, options);
    match spec {
        Ok(spec) => {
            let mut array_3d= imageproc::hog::cell_histograms(&img_gray, spec);
            let _view = array_3d.view_mut();
            log::info!("called cell_histograms");
        }
        Err(e) => {
            log::error!("HogSpec::from_options error: {:?}", e);
        }
    }
}

fn hog() {
    log::debug!("hog hog");
    // imageproc::hog::hog
}

fn render_hist_grid() {
    log::debug!("hog render_hist_grid");
    // imageproc::hog::render_hist_grid
}
