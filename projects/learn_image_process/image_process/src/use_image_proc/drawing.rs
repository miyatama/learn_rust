/**
 * drawingモジュールの機能を使い倒す
 * https://docs.rs/imageproc/0.25.0/imageproc/drawing/index.html
 * https://github.com/image-rs/imageproc/blob/master/examples/drawing.rs
 */

pub fn run() {
    let img = image::open("drawing_base.png").expect("failed to load image");

    let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    let img = imageproc::drawing::draw_line_segment(&img, (10.0, 10.0), (100.0, 10.0), red);

    img.save("./results/drawing_result.png")
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

    let font = ab_glyph::FontRef::try_from_slice(include_bytes!(
        "..\\..\\fonts\\ShipporiMincho-OTF-Medium.otf"
    ))
    .unwrap();

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
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        400,
        20,
        scale,
        &font,
        "draw_line_segment_mut",
    );
    imageproc::drawing::draw_line_segment_mut(&mut image, (400f32, 60f32), (600f32, 60f32), white);
    // はみ出してても描画自体は実施される
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (400f32, -120f32),
        (600f32, 90f32),
        white,
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (400f32, 120f32),
        (600f32, -120f32),
        white,
    );

    // draw hollow rect
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        20,
        200,
        scale,
        &font,
        "draw_hollowo_rect_mut",
    );
    imageproc::drawing::draw_hollow_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(20, 240).of_size(20, 20),
        white,
    );
    imageproc::drawing::draw_hollow_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(20, 270).of_size(20, 20),
        red,
    );
    imageproc::drawing::draw_hollow_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(-10, 300).of_size(100, 20),
        white,
    );

    // draw fill rect
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        500,
        200,
        scale,
        &font,
        "draw_fill_rect_mut",
    );
    // Draw a filled rect within bounds
    imageproc::drawing::draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(500, 240).of_size(20, 20),
        white,
    );
    imageproc::drawing::draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(500, 270).of_size(100, 20),
        red,
    );
    imageproc::drawing::draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(500, 300).of_size(600, 20),
        white,
    );

    // draw hollow circle
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        20,
        400,
        scale,
        &font,
        "draw_hollow_circle_mut",
    );
    imageproc::drawing::draw_hollow_circle_mut(&mut image, (27, 460), 15, white);
    imageproc::drawing::draw_hollow_circle_mut(&mut image, (40, 510), 20, red);
    imageproc::drawing::draw_hollow_circle_mut(&mut image, (-10, 560), 20, white);

    // draw fill circle
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        500,
        400,
        scale,
        &font,
        "draw_fill_circle_mut",
    );
    imageproc::drawing::draw_filled_circle_mut(&mut image, (527, 460), 15, white);
    imageproc::drawing::draw_filled_circle_mut(&mut image, (540, 510), 20, white);
    imageproc::drawing::draw_filled_circle_mut(&mut image, (990, 560), 20, white);

    // draw antialized line and polygon
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        20,
        600,
        scale,
        &font,
        "draw_antialized_x_mut",
    );
    imageproc::drawing::draw_antialiased_line_segment_mut(
        &mut image,
        (20, 630),
        (200, 630),
        white,
        |a, b, alpha| {
            image::Rgb([
                (a[0] as f32 * alpha + b[0] as f32 * (1.0 - alpha)) as u8,
                (a[1] as f32 * alpha + b[1] as f32 * (1.0 - alpha)) as u8,
                (a[2] as f32 * alpha + b[2] as f32 * (1.0 - alpha)) as u8,
            ])
        },
    );
    imageproc::drawing::draw_antialiased_polygon_mut(
        &mut image,
        &vec![
            imageproc::point::Point { x: 20, y: 650 },
            imageproc::point::Point { x: 50, y: 700 },
            imageproc::point::Point { x: 80, y: 650 },
            imageproc::point::Point { x: 150, y: 700 },
        ],
        white,
        |a, b, alpha| {
            image::Rgb([
                (a[0] as f32 * alpha + b[0] as f32 * (1.0 - alpha)) as u8,
                (a[1] as f32 * alpha + b[1] as f32 * (1.0 - alpha)) as u8,
                (a[2] as f32 * alpha + b[2] as f32 * (1.0 - alpha)) as u8,
            ])
        },
    );

    // draw polygon
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        500,
        600,
        scale,
        &font,
        "draw_polygon_mut",
    );
    imageproc::drawing::draw_polygon_mut(
        &mut image,
        &vec![
            imageproc::point::Point { x: 520, y: 650 },
            imageproc::point::Point { x: 550, y: 700 },
            imageproc::point::Point { x: 680, y: 650 },
            imageproc::point::Point { x: 650, y: 700 },
        ],
        white,
    );

    // draw polygon
    imageproc::drawing::draw_text_mut(
        &mut image,
        white,
        20,
        750,
        scale,
        &font,
        "draw_cubic_bezier_curve_mut",
    );
    imageproc::drawing::draw_cubic_bezier_curve_mut(
        &mut image,
        (20.0, 800.0),
        (200.0, 800.0),
        (80.0, 780.0),
        (150.0, 820.0),
        red,
    );

    image.save("./results/drawing_mut_result.png").unwrap();
}
