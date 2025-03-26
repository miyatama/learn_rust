pub fn run() {
    log::debug!("pixelops interpolate");
    let left = image::Rgb([10u8, 20u8, 30u8]);
    let right = image::Rgb([100u8, 80u8, 60u8]);

    let result = imageproc::pixelops::interpolate(left, right, 0.7);
    log::info!("interpolate result: {:?}", result);

    log::debug!("pixelops weighted_sum");
    let result = imageproc::pixelops::weighted_sum(left, right, 0.7, 0.3);
    log::info!("weighted_sum result: {:?}", result);
}
