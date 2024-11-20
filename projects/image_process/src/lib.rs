mod filter;
use self::filter::blur::apply as blur_apply;
use self::filter::blur::apply_multi as blur_apply_multi;
use log::debug;

pub fn run() {
    debug!("start run");
    debug!("start blur_apply");
    let img = image::open("example.jpg").expect("failed to load image");
    let blur_img = blur_apply(img);
    blur_img
        .save("blur.png")
        .expect("failed to save blur image");
    debug!("end blur_apply");

    debug!("start blur_apply_multi");
    let img = image::open("example.jpg").expect("failed to load image");
    let blur_img = blur_apply_multi(img);
    blur_img
        .save("blur_multi.png")
        .expect("failed to save blur image");
    debug!("end blur_apply_multi");
}
