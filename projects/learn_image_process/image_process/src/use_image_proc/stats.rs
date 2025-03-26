pub fn run() {
    cumulative_histogram();
    histogram();
    min_max();
    peak_signal_to_noise_ratio();
    percentile();
    root_mean_squared_error();
}

fn cumulative_histogram() {
    log::debug!("stats cumulative_histogram");
    let img = image::open("lena.png").expect("failed to load image");
    let image_rgb = img.clone().to_rgb8();
    let result = imageproc::stats::cumulative_histogram(&image_rgb);
    log::info!("cumulative_histogram result: {:?}", result.channels);
}

fn histogram() {
    log::debug!("stats histogram");
    let img = image::open("lena.png").expect("failed to load image");
    let image_rgb = img.clone().to_rgb8();
    let result = imageproc::stats::histogram(&image_rgb);
    log::info!("histogram result: {:?}", result.channels);
}

fn min_max() {
    log::debug!("stats min_max");
    let img = image::open("lena.png").expect("failed to load image");
    let image_rgb = img.clone().to_rgb8();
    let result = imageproc::stats::min_max(&image_rgb);
    log::info!("min_max result: {:?}", result);
}

fn peak_signal_to_noise_ratio() {
    log::debug!("stats peak_signal_to_noise_ratio");
    let img = image::open("lena.png").expect("failed to load image");
    let image_rgb = img.clone().to_rgb8();

    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let image_rgb_noized =
        imageproc::noise::salt_and_pepper_noise(&img.clone().to_rgb8(), rate, seed);

    let result = imageproc::stats::peak_signal_to_noise_ratio(&image_rgb, &image_rgb_noized);
    log::info!("peak_signal_to_noise_ratio result: {:?}", result);
}

fn percentile() {
    log::debug!("stats percentile");
    let img = image::open("lena.png").expect("failed to load image");
    let image_gray = img.clone().to_luma8();
    let p = 10u8; // p > 100
    let result = imageproc::stats::percentile(&image_gray, p);
    log::info!("percentile result: {:?}", result);
}

fn root_mean_squared_error() {
    log::debug!("stats root_mean_squared_error");
    let img = image::open("lena.png").expect("failed to load image");
    let image_rgb = img.clone().to_rgb8();

    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let image_rgb_noized =
        imageproc::noise::salt_and_pepper_noise(&img.clone().to_rgb8(), rate, seed);

    let result = imageproc::stats::root_mean_squared_error(&image_rgb, &image_rgb_noized);
    log::info!("root_mean_squared_error result: {:?}", result);
}
