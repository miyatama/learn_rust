pub fn run() {
    approximate_polygon_dp();
    arc_length();
    contour_area();
    convex_hull();
    min_area_rect();
    oriented_contour_area();
}

fn approximate_polygon_dp() {
    log::debug!("geometry approximate_polygon_dp");
    // https://docs.rs/imageproc/0.25.0/imageproc/point/struct.Point.html
    let curve = vec![
        (46, 16),
        (39, 29),
        (23, 30),
        (33, 45),
        (25, 67),
        (47, 54),
        (67, 64),
        (58, 44),
        (70, 32),
        (51, 32),
    ]
    .iter()
    .map(|point| imageproc::point::Point {
        x: point.0 as f32,
        y: point.1 as f32,
    })
    .collect::<Vec<_>>();
    let epsilon = 13f64;
    let closed = true;
    let result = imageproc::geometry::approximate_polygon_dp(&curve, epsilon, closed);

    let (width, height) = (100, 100);
    let mut img = image::RgbaImage::new(width, height);
    let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
    let green = image::Rgba([0u8, 255u8, 0u8, 255u8]);
    let mut current_point: Option<imageproc::point::Point<f32>> = None;
    curve.iter().for_each(|point| {
        if let Some(prev) = current_point {
            imageproc::drawing::draw_line_segment_mut(
                &mut img,
                (prev.x, prev.y),
                (point.x, point.y),
                red,
            );
        }
        current_point = Some(point.clone());
    });
    let start = curve[0].clone();
    let end = curve.iter().last().clone().unwrap();
    imageproc::drawing::draw_line_segment_mut(&mut img, (start.x, start.y), (end.x, end.y), red);

    let mut current_point: Option<imageproc::point::Point<f32>> = None;
    result.iter().for_each(|point| {
        if let Some(prev) = current_point {
            imageproc::drawing::draw_line_segment_mut(
                &mut img,
                (prev.x, prev.y),
                (point.x, point.y),
                green,
            );
        }
        current_point = Some(point.clone());
    });
    let start = result[0].clone();
    let end = result.iter().last().clone().unwrap();
    imageproc::drawing::draw_line_segment_mut(&mut img, (start.x, start.y), (end.x, end.y), green);
    img.save("./results/geometry_approximate_polygon_dp.png")
        .unwrap();
}

fn arc_length() {
    log::debug!("geometry arc_length");
}

fn contour_area() {
    log::debug!("geometry contour_area");
}

fn convex_hull() {
    log::debug!("geometry convex_hull");
}

fn min_area_rect() {
    log::debug!("geometry min_area_rect");
}

fn oriented_contour_area() {
    log::debug!("geometry oriented_contour_area");
}
