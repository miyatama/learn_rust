/**
 * drawingモジュールの機能を使い倒す
 * https://docs.rs/imageproc/0.25.0/imageproc/drawing/index.html
 * https://github.com/image-rs/imageproc/blob/master/examples/drawing.rs
 */

pub fn run() {
    let img = image::open("drawing_base.png").expect("failed to load image");

    let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    let img = imageproc::drawing::draw_line_segment(&img, (10.0, 10.0), (100.0, 12.0), red);

    img.save("drawing_result.png")
        .expect("failed to save drawing image");
}
