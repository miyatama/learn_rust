pub fn run() {
    as_blue_channel();
    as_green_channel();
    as_red_channel();
    blue_channel();
    green_channel();
    map_colors();
    map_colors2();
    map_colors_mut();
    map_pixels();
    map_pixels_mut();
    map_subpixels();
    map_subpixels_mut();
    red_channel();
}

fn as_blue_channel() {
    log::debug!("map as_blue_channel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::map::as_blue_channel(&img_gray);
    result.save("./results/map_as_blue_channel.png").unwrap();
}

fn as_green_channel() {
    log::debug!("map as_green_channel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::map::as_green_channel(&img_gray);
    result.save("./results/map_as_green_channel.png").unwrap();
}

fn as_red_channel() {
    log::debug!("map as_red_channel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::map::as_red_channel(&img_gray);
    result.save("./results/map_as_red_channel.png").unwrap();
}

fn blue_channel() {
    log::debug!("map blue_channel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_rgb = img.to_rgb8();
    let result = imageproc::map::blue_channel(&img_rgb);
    result.save("./results/map_blue_channel.png").unwrap();
}

fn green_channel() {
    log::debug!("map green_channel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_rgb = img.to_rgb8();
    let result = imageproc::map::green_channel(&img_rgb);
    result.save("./results/map_green_channel.png").unwrap();
}

fn map_colors() {
    log::debug!("map map_colors");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::map::map_colors(&img_gray, |pixel| {
        image::Rgb([
            pixel[0],
            (pixel[0] as f32 * 2.0 % 255.0) as u8,
            (pixel[0] as f32 * 3.0 % 255.0) as u8,
        ])
    });
    result.save("./results/map_map_colors.png").unwrap();
}

fn map_colors2() {
    log::debug!("map map_colors2");
    let img = image::open("lena.png").expect("failed to load image");
    let img_rgb = img.to_rgb8();
    let img_blue = imageproc::map::blue_channel(&img_rgb);
    let img_red = imageproc::map::red_channel(&img_rgb);
    let result = imageproc::map::map_colors2(&img_blue, &img_red, |p1, p2| {
        let r = (p1[0] as f32 + p2[0] as f32 % 255.0) as u8;
        let g = ((p1[0] as f32 + p2[0] as f32) * 2.0 % 255.0) as u8;
        let b = ((p1[0] as f32 + p2[0] as f32) * 3.0 % 255.0) as u8;
        image::Rgb([r, g, b])
    });
    result.save("./results/map_map_colors2.png").unwrap();
}

fn map_colors_mut() {
    log::debug!("map map_colors_mut");
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.to_luma8();
    imageproc::map::map_colors_mut(&mut img_gray, |pixel| {
        image::Luma([(pixel[0] as f32 * 2.0 % 255.0) as u8])
    });
    img_gray.save("./results/map_map_colors_mut.png").unwrap();
}

fn map_pixels() {
    log::debug!("map map_pixels");
    let img = image::open("lena.png").expect("failed to load image");
    let img_rgb = img.to_rgb8();
    let result = imageproc::map::map_pixels(&img_rgb, |x, y, pixel| {
        let r = pixel[0] as u8;
        let g = (x as f32 % 255.0) as u8;
        let b = (y as f32 % 255.0) as u8;
        image::Rgb([r, g, b])
    });
    result.save("./results/map_map_pixels.png").unwrap();
}

fn map_pixels_mut() {
    log::debug!("map map_pixels_mut");
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_rgb = img.to_rgb8();
    imageproc::map::map_pixels_mut(&mut img_rgb, |x, y, pixel| {
        let r = pixel[0] as u8;
        let g = (x as f32 % 255.0) as u8;
        let b = (y as f32 % 255.0) as u8;
        image::Rgb([r, g, b])
    });
    img_rgb.save("./results/map_map_pixels_mut.png").unwrap();
}

fn map_subpixels() {
    log::debug!("map map_subpixels");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.to_luma8();
    let result = imageproc::map::map_subpixels(&img_gray, |pixel| -2 * (pixel as i16));
    parse_to_lumau8(&result)
        .save("./results/map_map_subpixels.png")
        .unwrap();
}

fn map_subpixels_mut() {
    log::debug!("map map_subpixels_mut");
    let img = image::open("lena.png").expect("failed to load image");
    let mut img_gray = img.to_luma32f();
    imageproc::map::map_subpixels_mut(&mut img_gray, |pixel| -2.0 * pixel);
    parse_to_lumau8(&img_gray)
        .save("./results/map_map_subpixels_mut.png")
        .unwrap();
}

fn red_channel() {
    log::debug!("map red_channel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_rgb = img.to_rgb8();
    let result = imageproc::map::red_channel(&img_rgb);
    result.save("./results/map_red_channel.png").unwrap();
}

fn parse_to_lumau8<P: num_traits::Num + image::Primitive + Into<f32>>(
    img: &imageproc::definitions::Image<image::Luma<P>>,
) -> imageproc::definitions::Image<image::Luma<u8>> {
    let parse_to_f32 = |value: P| -> f32 { value.into() };
    let (width, height) = img.dimensions();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|pixel| parse_to_f32(pixel.0[0]))
        .collect::<Vec<f32>>();
    let min_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let max_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let limit = min_value.abs().max(max_value.abs());
    let pixels = pixels
        .iter()
        .map(|pixel| {
            let pixel = *pixel;
            let sign = if pixel < 0f32 { -1f32 } else { 1f32 };
            let pixel = pixel * sign;
            128f32 * (pixel / limit) * sign
        })
        .collect::<Vec<f32>>();
    let correction_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let correction_value = if correction_value < 0.0f32 {
        correction_value.abs()
    } else {
        0f32
    };
    let pixels = pixels
        .iter()
        .map(|value| {
            let value = *value;
            (value + correction_value) as u8
        })
        .collect::<Vec<u8>>();
    image::GrayImage::from_raw(width, height, pixels).unwrap()
}
