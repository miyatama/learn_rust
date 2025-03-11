/**
 * drawingモジュールの機能を使い倒す
 * https://docs.rs/imageproc/0.25.0/imageproc/drawing/index.html
 * https://github.com/image-rs/imageproc/blob/master/examples/drawing.rs
 */

pub fn run() {
    let img = image::open("drawing_base.png").expect("failed to load image");

    let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    let img = imageproc::drawing::draw_line_segment(&img, (10.0, 10.0), (100.0, 10.0), red);

    img.save("drawing_result.png")
        .expect("failed to save drawing image");

    mut_func();
}

fn mut_func() {
    let red = image::Rgb([255u8, 0u8, 0u8]);
    let green = image::Rgb([0u8, 255u8, 0u8]);
    let blue = image::Rgb([0u8, 0u8, 255u8]);
    let white = image::Rgb([255u8, 255u8, 255u8]);

    let mut image = image::RgbImage::new(1000, 1000);

    // Draw some crosses within bounds
    let scale = ab_glyph::PxScale::from(24.0);

    let font =
        ab_glyph::FontRef::try_from_slice(include_bytes!("..\\..\\fonts\\ShipporiMincho-OTF-Medium.otf")).unwrap();

    // draw_cross
    imageproc::drawing::draw_text_mut(&mut image, white, 20, 20, scale, &font, "draw_cross_mut");
    imageproc::drawing::draw_cross_mut(&mut image, white, 20, 60);
    imageproc::drawing::draw_cross_mut(&mut image, red, 20, 70);
    imageproc::drawing::draw_cross_mut(&mut image, blue, 20, 80);
    imageproc::drawing::draw_cross_mut(&mut image, green, 20, 90);
    // はみ出してても描画自体は実施される
    imageproc::drawing::draw_cross_mut(&mut image, white, 20, 1020);
    imageproc::drawing::draw_cross_mut(&mut image, white, 20, 0);

    // draw line
    imageproc::drawing::draw_text_mut(&mut image, white, 400, 20, scale, &font, "draw_line_segment_mut");
    imageproc::drawing::draw_line_segment_mut(&mut image, (400f32, 60f32), (600f32, 60f32), white);
    // はみ出してても描画自体は実施される
    imageproc::drawing::draw_line_segment_mut(&mut image, (400f32, -120f32), (600f32, 90f32), white);
    imageproc::drawing::draw_line_segment_mut(&mut image, (400f32, 120f32), (600f32, -120f32), white);

    // Draw a hollow rect within bounds
    imageproc::drawing::draw_hollow_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(60, 10).of_size(20, 20),
        white,
    );
    // Outside bounds
    imageproc::drawing::draw_hollow_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(300, 10).of_size(20, 20),
        white,
    );
    // Partially outside bounds
    imageproc::drawing::draw_hollow_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(90, -10).of_size(30, 20),
        white,
    );

    // Draw a filled rect within bounds
    imageproc::drawing::draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(130, 10).of_size(20, 20),
        white,
    );
    // Outside bounds
    imageproc::drawing::draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(300, 10).of_size(20, 20),
        white,
    );
    // Partially outside bounds
    imageproc::drawing::draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(180, -10).of_size(30, 20),
        white,
    );

    // Draw a hollow circle within bounds
    imageproc::drawing::draw_hollow_circle_mut(&mut image, (100, 100), 15, white);
    // Outside bounds
    imageproc::drawing::draw_hollow_circle_mut(&mut image, (400, 400), 20, white);
    // Partially outside bounds
    imageproc::drawing::draw_hollow_circle_mut(&mut image, (100, 190), 20, white);

    // Draw a filled circle within bounds
    imageproc::drawing::draw_filled_circle_mut(&mut image, (150, 100), 15, white);
    // Outside bounds
    imageproc::drawing::draw_filled_circle_mut(&mut image, (450, 400), 20, white);
    // Partially outside bounds
    imageproc::drawing::draw_filled_circle_mut(&mut image, (150, 190), 20, white);

    image.save("drawing_mut_result.png").unwrap();
}
