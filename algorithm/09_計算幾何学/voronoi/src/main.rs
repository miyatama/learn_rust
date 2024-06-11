use self::logics::commont::{self};
use self::logics::model_data::{self};
use self::logics::voronoi::{self};

pub mod logics;

fn main() {
    let width = 100.0;
    let height = 100.0;
    let points = vec![Point{id: 0 ,x: 50.0, y: 50.0}];
    let lines = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &point, &lines);
    println!("{}", &svg);

    let points = vec![Point{id: 0 ,x: 25.0, y: 50.0} ,Point{id: 0 ,x: 75.0, y: 50.0}];
    let lines = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &point, &lines);
    println!("{}", &svg);

    let points = vec![Point{id: 0 ,x: 50.0, y: 25.0} ,Point{id: 0 ,x: 50.0, y: 75.0}];
    let lines = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &point, &lines);
    println!("{}", &svg);

    let points = model_data::sample_data();
    let lines = voronoi::calc_voronoi_lines(width, height, &points);
    let svg = voronoi::create_svg(width, height, &point, &lines);
    println!("{}", &svg);
}
