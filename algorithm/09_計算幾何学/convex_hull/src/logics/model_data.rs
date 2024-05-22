use super::common::{self, Point};
use rand::Rng;

pub fn pentagram() -> Vec<Point> {
    vec![
        Point { x: 0.0, y: 100.0 },
        Point {
            x: -95.1057,
            y: 30.9017,
        },
        Point {
            x: -58.7785,
            y: -80.9017,
        },
        Point {
            x: 58.7785,
            y: -80.9017,
        },
        Point {
            x: 95.1057,
            y: 30.9017,
        },
    ]
}

pub fn pentagram_with_noise(noise_count: u32) -> Vec<Point> {
    let normal_data = pentagram();
    let max_x = normal_data
        .iter()
        .map(|point| point.x.abs())
        .fold(0.0f64, |m, v| m.max(v));
    let max_y = normal_data
        .iter()
        .map(|point| point.y.abs())
        .fold(0.0f64, |m, v| m.max(v));

    let mut rng = rand::thread_rng();
    let mut noises: Vec<Point> = Vec::new();
    loop {
        if noises.len() >= noise_count as usize {
            break;
        }

        let x = rng.gen_range(0.0..max_x);
        let y = rng.gen_range(0.0..max_y);
        let x_direction = rng.gen::<bool>();
        let y_direction = rng.gen::<bool>();
        let point_x = if x_direction { x } else { x * -1f64 };
        let point_y = if y_direction { y } else { y * -1f64 };
        let point = Point {
            x: point_x,
            y: point_y,
        };
        let in1 = common::in_triangle(&normal_data[1], &normal_data[0], &normal_data[4], &point);
        let in2 = common::in_triangle(&normal_data[1], &normal_data[4], &normal_data[3], &point);
        let in3 = common::in_triangle(&normal_data[1], &normal_data[3], &normal_data[2], &point);
        if in1 || in2 || in3 {
            noises.push(point.clone());
        }
    }

    noises.extend(normal_data);
    noises
}
