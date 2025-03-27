pub fn run() {
    log::debug!("utils");
    describe_pixel_diffs();
    gray_bench_image();
    load_image_or_panic();
    pixel_diff_summary();
    pixel_diffs();
    rgb_bench_image();
    significant_pixel_diff_summary();
}

fn describe_pixel_diffs() {
    log::debug!("utils describe_pixel_diffs");
    let img = image::open("lena.png").expect("failed to load image");
    let img_expect = img.clone().to_rgb8();

    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let img_actual = imageproc::noise::salt_and_pepper_noise(&img_expect, rate, seed);
    let diffs = imageproc::utils::pixel_diffs(
        &img_actual,
        &img_expect,
        |(x1, y1, pixel1), (x2, y2, pixel2)| x1 == x2 && y1 == y2 && pixel1.eq(&pixel2),
    );
    let result = imageproc::utils::describe_pixel_diffs(&img_actual, &img_expect, &diffs);
    let byte_start = result.char_indices().nth(0).unwrap().0;
    let byte_end = result.char_indices().nth(20).unwrap().0;

    let display_string: &str = &result[byte_start..byte_end];
    log::info!(
        "describe_pixel_diffs: {}, result at ({}, {})",
        display_string,
        byte_start,
        byte_end
    );
}

fn gray_bench_image() {
    log::debug!("utils gray_bench_image");
    let width = 100u32;
    let height = 255u32;
    let result = imageproc::utils::gray_bench_image(width, height);
    result.save("./results/utils_gray_bench_image.png").unwrap();
}

fn load_image_or_panic() {
    log::debug!("utils load_image_or_panic");
    let result = imageproc::utils::load_image_or_panic("./lena.png");
    result
        .save("./results/utils_load_image_or_panic.png")
        .unwrap();
}

fn pixel_diff_summary() {
    log::debug!("utils pixel_diff_summary");
    let img = image::open("lena.png").expect("failed to load image");
    let img_expect = img.clone().to_rgb8();

    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let img_actual = imageproc::noise::salt_and_pepper_noise(&img_expect, rate, seed);
    match imageproc::utils::pixel_diff_summary(&img_actual, &img_expect) {
        Some(value) => {
            log::info!("pixel_diff_summary diff size: {}", value.len());
        }
        None => {
            log::info!("pixel_diff_summary return none");
        }
    }
}

fn pixel_diffs() {
    log::debug!("utils pixel_diffs");
    let img = image::open("lena.png").expect("failed to load image");
    let img_expect = img.clone().to_rgb8();

    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let img_actual = imageproc::noise::salt_and_pepper_noise(&img_expect, rate, seed);
    let result = imageproc::utils::pixel_diffs(
        &img_actual,
        &img_expect,
        |(x1, y1, pixel1), (x2, y2, pixel2)| x1 == x2 && y1 == y2 && pixel1.eq(&pixel2),
    );
    let diff_size = result.len();
    log::info!(
        "pixel_diffs return: {:?}, len is {}",
        result[0..10]
            .iter()
            .map(|diff| (diff.x, diff.y))
            .collect::<Vec<(u32, u32)>>(),
        diff_size,
    );
}

fn rgb_bench_image() {
    log::debug!("utils rgb_bench_image");
    let width = 100u32;
    let height = 255u32;
    let result = imageproc::utils::rgb_bench_image(width, height);
    result.save("./results/utils_rgb_bench_image.png").unwrap();
}

fn significant_pixel_diff_summary() {
    log::debug!("utils significant_pixel_diff_summary");
    let img = image::open("lena.png").expect("failed to load image");
    let img_expect = img.clone().to_rgb8();

    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let img_actual = imageproc::noise::salt_and_pepper_noise(&img_expect, rate, seed);
    let result = imageproc::utils::significant_pixel_diff_summary(
        &img_actual,
        &img_expect,
        |(x1, y1, pixel1), (x2, y2, pixel2)| x1 == x2 && y1 == y2 && pixel1.eq(&pixel2),
    );
    log::info!("significant_pixel_diff_summary result: {:?}", result,);
}
