use self::logics::line_sweeper::{self};
use self::logics::model_data::{self};

pub mod logics;

fn main() {
    let lines = model_data::sample_data();
    let cross_points = line_sweeper::brute_force(&lines);
    line_sweeper::print_line_info(&lines);
    let result = line_sweeper::create_svg(&lines, &cross_points);
    eprintln!("{}", result);

    let lines = model_data::sample_data_with_noise(20);
    let cross_points = line_sweeper::brute_force(&lines);
    // graph_pinter::print(&lines, &cross_points);
}
