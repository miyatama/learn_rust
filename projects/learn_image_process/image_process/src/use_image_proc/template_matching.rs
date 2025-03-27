pub fn run() {
    log::debug!("template_matching");
    find_extremes();
    match_template();
    match_template_parallel();
    match_template_with_mask();
match_template_with_mask_parallel();
}

fn find_extremes() {
    log::debug!("template_matching find_extremes");
    let img = image::open("lena.png").expect("failed to load image");
    let img_gray = img.clone().to_luma8();
    let result = imageproc::template_matching::find_extremes(&img_gray);
    log::debug!("find_extremes result: {:?}", result);
}

fn match_template() {
    log::debug!("template_matching match_template");
    let img = image::open("lena.png").expect("failed to load image");
    let img_base = img.clone().to_luma8();
    let img = image::open("lena_face.png").expect("failed to load image");
    let img_face = img.clone().to_luma8();
    // https://docs.rs/imageproc/0.25.0/imageproc/template_matching/enum.MatchTemplateMethod.html
    let result = imageproc::template_matching::match_template(
        &img_base,
        &img_face,
        imageproc::template_matching::MatchTemplateMethod::SumOfSquaredErrors,
    );
    parse_to_lumau8(&result)
        .save("./results/template_matching_match_template.png")
        .unwrap();
}

fn match_template_parallel() {
    log::debug!("template_matching match_template_parallel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_base = img.clone().to_luma8();
    let img = image::open("lena_eyes.png").expect("failed to load image");
    let img_eye = img.clone().to_luma8();
    // https://docs.rs/imageproc/0.25.0/imageproc/template_matching/enum.MatchTemplateMethod.html
    let result = imageproc::template_matching::match_template_parallel(
        &img_base,
        &img_eye,
        imageproc::template_matching::MatchTemplateMethod::SumOfSquaredErrors,
    );
    parse_to_lumau8(&result)
        .save("./results/template_matching_match_template_parallel.png")
        .unwrap();
}

fn match_template_with_mask() {
    log::debug!("template_matching match_template_with_mask");
    let img = image::open("lena.png").expect("failed to load image");
    let img_base = img.clone().to_luma8();
    let img = image::open("lena_eyes.png").expect("failed to load image");
    let img_eye = img.clone().to_luma8();

    let mean = 100f64;
    let seddev = 20f64;
    let seed = 10u64;
    let img_eye_blur = imageproc::noise::gaussian_noise(&img_eye, mean, seddev, seed);

    // https://docs.rs/imageproc/0.25.0/imageproc/template_matching/enum.MatchTemplateMethod.html
    let result = imageproc::template_matching::match_template_with_mask(
        &img_base,
        &img_eye,
        imageproc::template_matching::MatchTemplateMethod::SumOfSquaredErrors,
        &img_eye_blur,
    );
    parse_to_lumau8(&result)
        .save("./results/template_matching_match_template_with_mask.png")
        .unwrap();
}

fn match_template_with_mask_parallel() {
    log::debug!("template_matching match_template_with_mask_parallel");
    let img = image::open("lena.png").expect("failed to load image");
    let img_base = img.clone().to_luma8();
    let img = image::open("lena_eyes.png").expect("failed to load image");
    let img_eye = img.clone().to_luma8();

    let mean = 100f64;
    let seddev = 20f64;
    let seed = 10u64;
    let img_eye_blur = imageproc::noise::gaussian_noise(&img_eye, mean, seddev, seed);

    // https://docs.rs/imageproc/0.25.0/imageproc/template_matching/enum.MatchTemplateMethod.html
    let result = imageproc::template_matching::match_template_with_mask_parallel(
        &img_base,
        &img_eye,
        imageproc::template_matching::MatchTemplateMethod::SumOfSquaredErrors,
        &img_eye_blur,
    );
    parse_to_lumau8(&result)
        .save("./results/template_matching_match_template_with_mask_parallel.png")
        .unwrap();
}

fn parse_to_lumau8<P: num_traits::Num + image::Primitive + Into<f64>>(
    img: &imageproc::definitions::Image<image::Luma<P>>,
) -> imageproc::definitions::Image<image::Luma<u8>> {
    let parse_to_f64 = |value: P| -> f64 { value.into() };
    let (width, height) = img.dimensions();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|pixel| parse_to_f64(pixel.0[0]))
        .collect::<Vec<f64>>();
    let min_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let max_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let limit = min_value.abs().max(max_value.abs());
    let pixels = pixels
        .iter()
        .map(|pixel| {
            let pixel = *pixel;
            let sign = if pixel < 0f64 { -1f64 } else { 1f64 };
            let pixel = pixel * sign;
            128f64 * (pixel / limit) * sign
        })
        .collect::<Vec<f64>>();
    let correction_value = pixels.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    let correction_value = if correction_value < 0.0f64 {
        correction_value.abs()
    } else {
        0f64
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
