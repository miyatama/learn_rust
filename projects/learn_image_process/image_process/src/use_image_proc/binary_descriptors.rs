/**
 * https://docs.rs/imageproc/0.25.0/imageproc/binary_descriptors/index.html
 */

pub fn run() {
    let img = image::open("lena.png").expect("failed to load image");
    let img_base = img.to_luma8();
    let keypoints = get_keypoints(&img_base);
    let length = 128usize;
    let override_test_pairs = None;
    let base_brief = imageproc::binary_descriptors::brief::brief(
        &img_base,
        &keypoints,
        length,
        override_test_pairs,
    );

    let img = image::open("lena_face.png").expect("failed to load image");
    let img_parts = img.to_luma8();
    let keypoints = get_keypoints(&img_parts);
    let length = 128usize;
    let override_test_pairs = None;
    let parts_brief = imageproc::binary_descriptors::brief::brief(
        &img_parts,
        &keypoints,
        length,
        override_test_pairs,
    );

    match (base_brief, parts_brief) {
        (Ok(base), Ok(parts)) => {
            log::info!(
                "base descriptor len: {}, test pair len: {}",
                base.0.len(),
                base.1.len()
            );
            log::info!(
                "parts descriptor len: {}, test pair len: {}",
                parts.0.len(),
                parts.1.len()
            );
            let base_descriptor = base.0;
            let parts_descriptor = parts.0;
            let threshold = 300u32;
            let seed = Some(44u64);
            let match_result = imageproc::binary_descriptors::match_binary_descriptors(
                &base_descriptor,
                &parts_descriptor,
                threshold,
                seed,
            );

            let img = image::open("lena.png").expect("failed to load image");
            let img_base = img.to_rgba8();
            let (base_width, base_height) = img_base.dimensions();
            let img_parts = image::open("lena_face.png").expect("failed to load image");
            let img_parts = img_parts.to_rgba8();
            let (parts_width, parts_height) = img_parts.dimensions();
            let mut result_pixels: Vec<image::Rgba<u8>> =
                Vec::with_capacity((base_width * base_height) as usize);
            for y in 0..base_height {
                for x in 0..base_width {
                    if y < parts_height && x < parts_width {
                        result_pixels.push(*img_parts.get_pixel(x, y));
                    } else {
                        result_pixels.push(*img_base.get_pixel(x, y));
                    }
                }
            }
            let result_pixels = result_pixels
                .into_iter()
                .map(|rgba| vec![rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]])
                .flatten()
                .collect::<Vec<u8>>();
            let mut img_result =
                image::ImageBuffer::from_raw(base_width, base_height, result_pixels).unwrap();
            let red = image::Rgba([255u8, 0u8, 0u8, 255u8]);
            match_result.iter().for_each(|pair| {
                let (base, parts) = pair;
                let start = (base.corner.x as i32, base.corner.y as i32);
                let end = (parts.corner.x as i32, parts.corner.y as i32);
                let radius = 2;
                imageproc::drawing::draw_hollow_circle_mut(&mut img_result, start, radius, red);
                imageproc::drawing::draw_hollow_circle_mut(&mut img_result, end, radius, red);
                let start = (start.0 as f32, start.1 as f32);
                let end = (end.0 as f32, end.1 as f32);
                imageproc::drawing::draw_line_segment_mut(&mut img_result, start, end, red);
            });
            img_result
                .save("./results/binary_descriptors_match_binary_descriptors.png")
                .unwrap();
        }
        (base, parts) => {
            if let Err(base_error) = base {
                log::error!("base descriptor error: {}", base_error);
            }
            if let Err(parts_error) = parts {
                log::error!("parts descriptor error: {}", parts_error);
            }
        }
    }
}

fn get_keypoints(img: &image::GrayImage) -> Vec<imageproc::point::Point<u32>> {
    let (width, height) = img.dimensions();
    let (max_width, max_height) = (width - 16, height - 16);
    let corners = imageproc::corners::corners_fast9(&img, 80);
    corners
        .iter()
        .map(|corner| imageproc::point::Point {
            x: corner.x,
            y: corner.y,
        })
        .filter(|point| point.x > 16 && point.x < max_width && point.y > 16 && point.y < max_height)
        .collect::<Vec<_>>()
}
