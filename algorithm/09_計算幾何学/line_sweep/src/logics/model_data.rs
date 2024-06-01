use super::common::{Line, Point};
use rand::Rng;

pub fn sample_data() -> Vec<Line> {
    vec![
        Line {
            p1: Point { x: -5.1, y: 5.1 },
            p2: Point { x: -5.5, y: -5.2 },
        },
        Line {
            p1: Point { x: -4.0, y: 7.0 },
            p2: Point { x: -4.0, y: 3.0 },
        },
        Line {
            p1: Point { x: -1.0, y: 9.0 },
            p2: Point { x: -9.0, y: 1.0 },
        },
        Line {
            p1: Point { x: -7.0, y: 1.5 },
            p2: Point { x: 3.0, y: -4.0 },
        },
        Line {
            p1: Point { x: -1.0, y: -6.0 },
            p2: Point { x: 5.5, y: 2.0 },
        },
        Line {
            p1: Point { x: 7.0, y: 7.0 },
            p2: Point { x: 7.0, y: 3.0 },
        },
    ]
}

pub fn sample_data_with_noise(noise_count: u32) -> Vec<Line> {
    let normal_data = sample_data();
    let values = normal_data
        .iter()
        .map(|line| {
            vec![
                line.p1.x.abs(),
                line.p1.y.abs(),
                line.p2.x.abs(),
                line.p2.y.abs(),
            ]
        })
        .flatten()
        .collect::<Vec<f64>>();
    let range = values.iter().fold(f64::MIN, |m, v| m.max(*v));

    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for i in 0..(noise_count * 2) {
        let x = rng.gen_range(0.0..range);
        let y = rng.gen_range(0.0..range);
        let x_direction = rng.gen::<bool>();
        let y_direction = rng.gen::<bool>();
        let point_x = if x_direction { x } else { x * -1f64 };
        let point_y = if y_direction { y } else { y * -1f64 };
        points.push(Point {
            x: point_x,
            y: point_y,
        });
    }
    let mut noises: Vec<Line> = Vec::new();
    for i in (0..points.len()).step_by(2) {
        noises.push(Line {
            p1: points[i].clone(),
            p2: points[i + 1].clone(),
        });
    }

    noises.extend(normal_data);
    noises
}
