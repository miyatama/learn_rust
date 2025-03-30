/**
 * https://docs.rs/imageproc/0.25.0/imageproc/contours/index.html
 */
pub fn run() {
    log::info!("imageproc contours module");
    let img = image::open("contour_base.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let threshold = 100u8;
    let img_gray = imageproc::contrast::threshold(
        &img_gray,
        threshold,
        imageproc::contrast::ThresholdType::Binary,
    );
    img_gray
        .save("./results/contours_find_contours_threshold.png")
        .unwrap();
    log::info!("contours find_contours");
    let contours = imageproc::contours::find_contours::<u32>(&img_gray);
    log::info!("contours length: {}", contours.len());
    let red = image::Rgb([255u8, 0u8, 0u8]);
    let mut img_result = img.clone().to_rgb8();
    contours.iter().for_each(|contour| {
        let mut prev: Option<imageproc::point::Point<u32>> = None;
        contour.points.iter().for_each(|point| {
            if let Some(prev_point) = prev {
                let start = (prev_point.x as f32, prev_point.y as f32);
                let end = (point.x as f32, point.y as f32);
                imageproc::drawing::draw_line_segment_mut(&mut img_result, start, end, red);
            }
            prev = Some(point.clone());
        });
        if let Some(prev_point) = prev {
            let start = (prev_point.x as f32, prev_point.y as f32);
            let end = (contour.points[0].x as f32, contour.points[0].y as f32);
            imageproc::drawing::draw_line_segment_mut(&mut img_result, start, end, red);
        }
    });
    img_result
        .save("./results/contours_find_contours.png")
        .unwrap();

    log::info!("contours find_contours_with_threshold");
    let contours = imageproc::contours::find_contours_with_threshold::<u32>(&img_gray, threshold);
    log::info!("contours length: {}", contours.len());
    let mut img_result = img.clone().to_rgb8();
    contours.iter().for_each(|contour| {
        let mut prev: Option<imageproc::point::Point<u32>> = None;
        contour.points.iter().for_each(|point| {
            if let Some(prev_point) = prev {
                let start = (prev_point.x as f32, prev_point.y as f32);
                let end = (point.x as f32, point.y as f32);
                imageproc::drawing::draw_line_segment_mut(&mut img_result, start, end, red);
            }
            prev = Some(point.clone());
        });
        if let Some(prev_point) = prev {
            let start = (prev_point.x as f32, prev_point.y as f32);
            let end = (contour.points[0].x as f32, contour.points[0].y as f32);
            imageproc::drawing::draw_line_segment_mut(&mut img_result, start, end, red);
        }
    });
    img_result
        .save("./results/contours_find_contours_with_threshold.png")
        .unwrap();
}
