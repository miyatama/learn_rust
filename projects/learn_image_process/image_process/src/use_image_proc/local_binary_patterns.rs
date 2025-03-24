pub fn run() {
    log::debug!("local_binary_patterns count_transitions");
    let value = 0b10110010;
    let result = imageproc::local_binary_patterns::count_transitions(value);
    log::info!(
        "bit value {:08b} count_transitions result is {}",
        value,
        result
    );

    let img = image::open("lena.png").expect("failed to load image");
    let gray_image_buffer = img.to_luma8();
    let (x, y) = (100, 100);
    let result = imageproc::local_binary_patterns::local_binary_pattern(&gray_image_buffer, x, y);

    log::info!(
        "local_binary_pattern ({}, {}) result is {:08b}",
        x,
        y,
        result.unwrap()
    );

    let value = 0b10110100;
    let result = imageproc::local_binary_patterns::min_shift(value);
    log::info!("bit value {:08b} min_shift result is {:08b}", value, result);
}
