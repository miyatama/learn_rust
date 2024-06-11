use super::common::{Line, Point};
use std::collections::VecDeque;
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Line as SvgLine;
use svg::node::element::Rectangle as SvgRectangle;
use svg::Document;

#[derive(Debug)]
struct Event {
    site: Option<Point>,
    event_type: EventType,
}

#[derive(Debug)]
enum EventType {
    Site,
    Circle,
}

/**
 * 放物線の弧(汀線の要素)
 */
#[derive(Debug)]
struct Arc {
    focal_point: Point,
    // 準線のy位置
    sub_line: f64,
}

impl Arc {
    /**
     * 放物線の最低点を返す
     */
    pub fn get_v(&self) -> f64 {
        (self.sub_line - self.focal_point.y) / 2.0
    }

    pub fn get_cross_points(&self, b: Arc) -> f64 {
        let p = self.get_v();
        // 4p(y - k) = (x - h)^2 より
        // y = (x^2 - 2xh + h^2 + 4pk) / 4p
        // k = self.focal_point.y
        // h = self.focal_point.x
        0.0
    }

    pub fn get_x_range(&self, x_upper_limit: f64) -> (f64, f64) {
        // 焦点のy位置でx範囲を計算する
        // 4p(y - k) = (x - h)^2 より
        // x = sqrt(4p(y - k)) + h
        // x = h
        //   k = self.focal_point.y
        //   h = self.focal_point.x
        //   y = self.focal_point.y
        (0.0, 0.0)
    }
}

/**
 * 点の集合からボロノイ辺を作成する
 */
pub fn calc_voronoi_lines(width: f64, height: f64, points: &Vec<Point>) -> Vec<Line> {
    let mut points = points.clone();
    if points.len() <= 1 {
        return create_point_one_voronoi(width, height);
    } else if points.len() <= 2 {
        return create_point_twe_voronoi(width, height, &points);
    }
    points.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    // サイトイベントを登録
    let mut queue: VecDeque<Event> = VecDeque::new();
    points.iter().for_each(|point| {
        queue.push_back(Event {
            site: Some(point.clone()),
            event_type: EventType::Site,
        })
    });

    let base_line_y: f64 = f64::MIN;
    let beach_line: Vec<Arc> = Vec::new();
    while let Some(event) = queue.pop_front() {
        match (event.event_type, event.site) {
            (EventType::Site, Some(site)) => {
                // 汀線追加

                // 基準線の更新
            }
            (EventType::Circle, _) => {}
            _ => {}
        }
    }
    vec![]
}

fn create_point_one_voronoi(width: f64, height: f64) -> Vec<Line> {
    create_rect_lines(&vec![
        Point {
            id: 1,
            x: 0.0,
            y: 0.0,
        },
        Point {
            id: 1,
            x: width,
            y: 0.0,
        },
        Point {
            id: 1,
            x: width,
            y: height,
        },
        Point {
            id: 1,
            x: 0.0,
            y: height,
        },
    ])
}

fn create_point_twe_voronoi(width: f64, height: f64, points: &Vec<Point>) -> Vec<Line> {
    let delta_y = get_delta(points[0].y, points[1].y);
    let delta_x = get_delta(points[0].x, points[1].x);
    let min_y = min_f64(points[0].y, points[1].y);
    let min_x = min_f64(points[0].x, points[1].x);
    match (delta_y, delta_x) {
        (0.0, _) => {
            // 縦線
            let center_x = delta_x / 2.0 + min_x;
            let mut left_lines = create_rect_lines(&vec![
                Point {
                    id: 0,
                    x: 0.0,
                    y: 0.0,
                },
                Point {
                    id: 0,
                    x: center_x,
                    y: 0.0,
                },
                Point {
                    id: 0,
                    x: center_x,
                    y: height,
                },
                Point {
                    id: 0,
                    x: 0.0,
                    y: height,
                },
            ]);
            let right_lines = create_rect_lines(&vec![
                Point {
                    id: 0,
                    x: center_x,
                    y: 0.0,
                },
                Point {
                    id: 0,
                    x: width,
                    y: 0.0,
                },
                Point {
                    id: 0,
                    x: width,
                    y: height,
                },
                Point {
                    id: 0,
                    x: center_x,
                    y: height,
                },
            ]);
            left_lines.extend(right_lines);
            left_lines
        }
        (_, 0.0) => {
            // 横線
            let center_y = delta_y / 2.0 + min_y;
            let mut top_lines = create_rect_lines(&vec![
                Point {
                    id: 0,
                    x: 0.0,
                    y: 0.0,
                },
                Point {
                    id: 0,
                    x: width,
                    y: 0.0,
                },
                Point {
                    id: 0,
                    x: width,
                    y: center_y,
                },
                Point {
                    id: 0,
                    x: 0.0,
                    y: center_y,
                },
            ]);
            let bottom_lines = create_rect_lines(&vec![
                Point {
                    id: 0,
                    x: 0.0,
                    y: center_y,
                },
                Point {
                    id: 0,
                    x: width,
                    y: center_y,
                },
                Point {
                    id: 0,
                    x: width,
                    y: height,
                },
                Point {
                    id: 0,
                    x: 0.0,
                    y: height,
                },
            ]);
            top_lines.extend(bottom_lines);
            top_lines
        }
        (y, x) => {
            // 線対象
            let a = -1.0 / (delta_y / delta_x);
            let center_y = delta_y / 2.0 + min_y;
            let center_x = delta_x / 2.0 + min_x;
            let b = a * center_x - center_y;
            let max_y = min_f64(height, b);
            let min_y = max_f64(0.0, a * width + b);
            let max_x = (max_y - b) / a;
            let min_x = (min_y - b) / a;

            // 斜め45度線のみ三角形に分離する
            if is_triangle_line(width, height, min_x, min_y, max_x, max_y) {
                if min_x == 0.0 && min_y == height && max_x == width && max_y == 0.0 {
                    // 右肩下がり
                    let mut left_lines = create_triangle_lines(&vec![
                        Point {
                            id: 0,
                            x: 0.0,
                            y: 0.0,
                        },
                        Point {
                            id: 1,
                            x: 0.0,
                            y: height,
                        },
                        Point {
                            id: 3,
                            x: width,
                            y: 0.0,
                        },
                    ]);
                    let righg_lines = create_triangle_lines(&vec![
                        Point {
                            id: 0,
                            x: 0.0,
                            y: height,
                        },
                        Point {
                            id: 1,
                            x: width,
                            y: height,
                        },
                        Point {
                            id: 3,
                            x: width,
                            y: 0.0,
                        },
                    ]);
                    left_lines.extend(righg_lines);
                    left_lines
                } else {
                    // 右肩上がり
                    let mut left_lines = create_triangle_lines(&vec![
                        Point {
                            id: 0,
                            x: 0.0,
                            y: 0.0,
                        },
                        Point {
                            id: 1,
                            x: width,
                            y: height,
                        },
                        Point {
                            id: 3,
                            x: width,
                            y: 0.0,
                        },
                    ]);
                    let righg_lines = create_triangle_lines(&vec![
                        Point {
                            id: 0,
                            x: 0.0,
                            y: height,
                        },
                        Point {
                            id: 1,
                            x: width,
                            y: height,
                        },
                        Point {
                            id: 3,
                            x: 0.0,
                            y: 0.0,
                        },
                    ]);
                    left_lines.extend(righg_lines);
                    left_lines
                }
            } else {
                // 台形2つに分離
                vec![]
            }
        }
    }
}

fn create_triangle_lines(points: &Vec<Point>) -> Vec<Line> {
    vec![
        Line {
            id: 0,
            p1: points[0].clone(),
            p2: points[1].clone(),
        },
        Line {
            id: 1,
            p1: points[1].clone(),
            p2: points[2].clone(),
        },
        Line {
            id: 2,
            p1: points[2].clone(),
            p2: points[0].clone(),
        },
    ]
}

fn create_rect_lines(points: &Vec<Point>) -> Vec<Line> {
    vec![
        Line {
            id: 0,
            p1: points[0].clone(),
            p2: points[1].clone(),
        },
        Line {
            id: 1,
            p1: points[1].clone(),
            p2: points[2].clone(),
        },
        Line {
            id: 2,
            p1: points[2].clone(),
            p2: points[3].clone(),
        },
        Line {
            id: 2,
            p1: points[3].clone(),
            p2: points[0].clone(),
        },
    ]
}

fn is_triangle_line(
    width: f64,
    height: f64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> bool {
    // 右肩下がりの斜線
    if min_x == 0.0 && min_y == height && max_x == width && max_y == 0.0 {
        return true;
    }

    // 右肩上がりの斜線
    if min_x == 0.0 && min_y == 0.0 && max_x == width && max_y == height {
        return true;
    }

    false
}

fn min_f64(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn get_delta(a: f64, b: f64) -> f64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/**
 * 線分と交点を元にSVG文字列を生成
 * see: https://zenn.dev/tipstar0125/articles/d2cf0ef63bceb7
 */
pub fn create_svg(width: f64, height: f64, points: &Vec<Point>, lines: &Vec<Line>) -> String {
    let svg_size = 600i64;
    let n = 20i64;
    let margin = 10i64;
    let line_color = "#121212";
    let point_color = "#fc1212";
    let mut svg = Document::new()
        .set(
            "viewBox",
            (
                -margin,
                -margin,
                (svg_size + 2 * margin) as usize,
                (svg_size + 2 * margin) as usize,
            ),
        )
        .set("width", (svg_size + margin) as usize)
        .set("height", (svg_size + margin) as usize)
        .set("style", "background-color:#F2F3F5");

    // グラフの外枠
    svg = svg.add(
        SvgRectangle::new()
            .set("x", 10)
            .set("y", 10)
            .set("width", 580)
            .set("height", 580)
            .set("fill", "#F5F5F5")
            .set("stroke", line_color)
            .set("stroke-width", 3),
    );

    // 線分の描画
    // 線分の最小最大からx, yの範囲を求める
    let max_range = lines
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
        .fold(0.0f64, |m, v| m.max(v));
    let graph_unit = (svg_size - (margin * 3 * 2)) as f64 / (max_range * 2.0);
    let change_coordinate =
        |x: f64, y: f64, range: f64, graph_unit: f64, margin: usize| -> (usize, usize) {
            let x = x + range;
            let y = if y > 0.0 {
                (y - range).abs()
            } else {
                y.abs() + range
            };
            (
                (x * graph_unit) as usize + margin,
                (y * graph_unit) as usize + margin,
            )
        };
    let graph_margin = (margin * 3) as usize;
    eprintln!("graph unit: {}, shape range: {}", graph_unit, max_range);
    for i in 0..lines.len() {
        let line = &lines[i];
        let (x1, y1) = change_coordinate(line.p1.x, line.p1.y, max_range, graph_unit, graph_margin);
        let (x2, y2) = change_coordinate(line.p2.x, line.p2.y, max_range, graph_unit, graph_margin);
        svg = svg.add(get_svg_line(x1, y1, x2, y2, line_color));
    }

    for i in 0..points.len() {
        let point = &points[i];
        let (x, y) = change_coordinate(point.x, point.y, max_range, graph_unit, graph_margin);
        svg = svg.add(get_svg_circle(x, y, point_color));
    }

    svg.to_string()
}

fn get_svg_line(x1: usize, y1: usize, x2: usize, y2: usize, line_color: &str) -> SvgLine {
    SvgLine::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", line_color)
        .set("stroke-width", 3)
        .set("stroke-linecap", "round")
}

fn get_svg_circle(x: usize, y: usize, color: &str) -> SvgCircle {
    SvgCircle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", 3)
        .set("fill", color)
}
