pub fn run() {
    gaussian_noise();
    gaussian_noise_mut();
    salt_and_pepper_noise();
    salt_and_pepper_noise_mut();
}

fn gaussian_noise() {
    log::debug!("noise gaussian_noise");
    let img = image::open("lena.png").expect("failed to load image");
    let image_buffer = img.to_rgb8();
    let mean = 100f64;
    let seddev = 20f64;
    let seed = 10u64;
    let result = imageproc::noise::gaussian_noise(&image_buffer, mean, seddev, seed);
    result.save("./results/noise_gaussian_noise.png").unwrap();
}

fn gaussian_noise_mut() {
    log::debug!("noise gaussian_noise_mut");
    let img = image::open("lena.png").expect("failed to load image");
    let mut image_buffer = img.to_rgb8();
    let mean = 125f64;
    let seddev = 30f64;
    let seed = 15u64;
    imageproc::noise::gaussian_noise_mut(&mut image_buffer, mean, seddev, seed);
    image_buffer
        .save("./results/noise_gaussian_noise_mut.png")
        .unwrap();
}

fn salt_and_pepper_noise() {
    log::debug!("noise salt_and_pepper_noise");
    let img = image::open("lena.png").expect("failed to load image");
    let image_buffer = img.to_rgb8();
    let rate = 0.2f64; // rage 0.0 between 1.0
    let seed = 10u64;
    let result = imageproc::noise::salt_and_pepper_noise(&image_buffer, rate, seed);
    result
        .save("./results/noise_salt_and_pepper_noise.png")
        .unwrap();
}

fn salt_and_pepper_noise_mut() {
    log::debug!("noise salt_and_pepper_noise_mut");
    let img = image::open("lena.png").expect("failed to load image");
    let mut image_buffer = img.to_rgb8();
    let rate = 0.3f64;
    let seed = 30u64;
    imageproc::noise::salt_and_pepper_noise_mut(&mut image_buffer, rate, seed);
    image_buffer
        .save("./results/noise_salt_and_pepper_noise_mut.png")
        .unwrap();
}
