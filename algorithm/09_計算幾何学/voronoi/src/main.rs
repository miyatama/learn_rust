use self::logics::common::Point;
use self::logics::model_data::{self};
use self::logics::voronoi::{self};

pub mod logics;

fn main() {
    /*
    let width = 100.0;
    let height = 100.0;
    let points = model_data::sample_data_one();
    let polygons = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &points, &polygons);
    println!("{}", &svg);
     */

    /*
    let width = 100.0;
    let height = 100.0;
    let points = model_data::sample_data_two();
    let polygons = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &points, &polygons);
    println!("{}", &svg);
      */

    // 3点
    let width = 100.0;
    let height = 100.0;
    let points = model_data::sample_data_three();
    let polygons = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &points, &polygons);
    println!("{}", &svg);

    /*
    // 4点
    let width = 100.0;
    let height = 100.0;
    let points = model_data::sample_data_four();
    let polygons = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &points, &polygons);
    println!("{}", &svg);

    // 13点
    let width = 100.0;
    let height = 100.0;
    let points = model_data::sample_data();
    let polygons = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &points, &polygons);
    println!("{}", &svg);
     */
}
