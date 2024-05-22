use self::logics::common::{self};
use self::logics::model_data::{self};

pub mod logics;

fn main() {
    let points = model_data::pentagram();
    common::print_points("pentagram".to_string(), &points);
    let points = model_data::pentagram_with_noise(20);
    common::print_points("pentagram with noise".to_string(), &points);
}
