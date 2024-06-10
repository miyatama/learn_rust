use self::logics::commont::{self};
use self::logics::model_data::{self};
use self::logics::voronoi::{self};

pub mod logics;

fn main() {
    let points = model_data::sample_data();
    let width = 100.0;
    let height = 100.0;
    let lines = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &point, &lines);
    println!("{}", &svg);
}
