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
}
