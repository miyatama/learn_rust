use self::logics::line_sweeper::{self};
use self::logics::model_data::{self};

pub mod logics;

fn main() {
    let lines = model_data::sample_data();
    let cross_points = line_sweeper::brute_force(&lines);
    // graph_pinter::print(&lines, &cross_points);
    let lines = model_data::sample_data_with_noise(20);
    let cross_points = line_sweeper::brute_force(&lines);
    // graph_pinter::print(&lines, &cross_points);
}
