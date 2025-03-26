mod filter;
mod use_image_proc;
mod util;
use self::filter::blur::apply as blur_apply;
use self::filter::resize::apply as scale_apply;
use self::filter::rotate::apply as rotate_apply;
use self::use_image_proc::contours::run as image_proc_contours_run;
use self::use_image_proc::contrast::run as image_proc_contrast_run;
use self::use_image_proc::corners::run as image_proc_corners_run;
use self::use_image_proc::distance_transform::run as imageproc_distance_transform_run;
use self::use_image_proc::drawing::run as image_proc_drawing_run;
use self::use_image_proc::edges::run as image_proc_edges_run;
use self::use_image_proc::filter::run as image_proc_filter_run;
use self::use_image_proc::geometric_transformations::run as image_proc_geometric_transformations_run;
use self::use_image_proc::geometry::run as image_proc_geometry_run;
use self::use_image_proc::gradients::run as image_proc_gradients_run;
use self::use_image_proc::haar::run as image_proc_haar_run;
use self::use_image_proc::hog::run as image_proc_hog_run;
use self::use_image_proc::hough::run as image_proc_hough_run;
use self::use_image_proc::integral_image::run as image_proc_integral_image_run;
use self::use_image_proc::local_binary_patterns::run as image_proc_local_binary_patterns_run;
use self::use_image_proc::map::run as image_proc_map_run;
use self::use_image_proc::math::run as image_proc_math_run;
use self::use_image_proc::morphology::run as image_proc_morphology_run;
use self::use_image_proc::noise::run as image_proc_noise_run;
use self::use_image_proc::pixelops::run as image_proc_pixelops_run;
use self::use_image_proc::region_labelling::run as image_proc_region_labelling_run;
use self::use_image_proc::seam_carving::run as image_proc_region_seam_carving_run;

use log::debug;

pub fn run() {
    debug!("start run");
    debug!("start blur_apply");
    let img = image::open("example.jpg").expect("failed to load image");
    let blur_img = blur_apply(img);
    blur_img
        .save("./results/self_blur.png")
        .expect("failed to save blur image");
    debug!("end blur_apply");

    /*
    debug!("start blur_apply_multi");
    let img = image::open("example.jpg").expect("failed to load image");
    let blur_img = blur_apply_multi(img);
    blur_img
        .save("blur_multi.png")
        .expect("failed to save blur image");
    debug!("end blur_apply_multi");
     */

    let img = image::open("example.jpg").expect("failed to load image");
    let rotate_img = rotate_apply(img, 45i32);
    match rotate_img.unwrap() {
        Ok(img) => {
            img.save("./results/self_rotate_img.png")
                .expect("failed to save rotate image");
        }
        _ => {
            log::error!("rotate process failed");
        }
    }
    // TODO override

    let img = image::open("example.jpg").expect("failed to load image");
    let scaled_img = scale_apply(img, 1.5f64);
    match scaled_img.unwrap() {
        Ok(img) => {
            img.save("./results/self_scaled_img.png")
                .expect("failed to save scaled image");
        }
        _ => {
            log::error!("scale process failed");
        }
    }
    image_proc_drawing_run();
    image_proc_filter_run();
    image_proc_contours_run();
    image_proc_geometric_transformations_run();
    image_proc_contrast_run();
    image_proc_corners_run();
    imageproc_distance_transform_run();
    image_proc_edges_run();
    image_proc_geometry_run();
    image_proc_gradients_run();
    image_proc_haar_run();
    image_proc_hog_run();
    image_proc_hough_run();
    image_proc_integral_image_run();
    image_proc_local_binary_patterns_run();
    image_proc_map_run();
    image_proc_math_run();
    image_proc_morphology_run();
    image_proc_noise_run();
    image_proc_pixelops_run();
    image_proc_region_labelling_run();
    image_proc_region_seam_carving_run();
}
