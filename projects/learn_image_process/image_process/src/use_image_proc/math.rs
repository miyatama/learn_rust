pub fn run() {
    l1_norm();
    l2_norm();
}

fn l1_norm() {
    log::debug!("math l1_norm");
    let value = vec![5f32, 10f32, 25f32];
    let result = imageproc::math::l1_norm(&value);
    log::info!("value {:?} l1 norm is {}", value, result);
}
fn l2_norm() {
    log::debug!("math l2_norm");
    let value = vec![5f32, 10f32, 25f32];
    let result = imageproc::math::l2_norm(&value);
    log::info!("value {:?} l2 norm is {}", value, result);
}
