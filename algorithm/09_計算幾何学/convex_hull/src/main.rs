use self::logics::common::{self};
use self::logics::convex_hull_scan::{self};
use self::logics::model_data::{self};

pub mod logics;

fn main() {
    let points = model_data::pentagram_with_noise(20);
    common::print_points("pentagram with noise".to_string(), &points);
    let convec_hull = convex_hull_scan::scan(&points);
    common::print_points("convec hull scan".to_string(), &convec_hull);
}
