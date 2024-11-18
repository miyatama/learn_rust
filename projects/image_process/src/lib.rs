mod filter;
use self::filter::blur::apply as blur_apply;

pub fn run() {
    let img = image::open("example.jpg").expect("failed to load image");
    let blur_img = blur_apply(img);
    blur_img
        .save("blur.png")
        .expect("failed to save blur image");
}
