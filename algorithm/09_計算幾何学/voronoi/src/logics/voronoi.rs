use super::common::{Line, LinePointDirection, Point, Polygon};
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
    // 焦点
    focal_point: Point,
}

impl Arc {
    /**
     * 放物線の最低点を返す
     */
    pub fn get_v(&self, sub_line: f64) -> f64 {
        (sub_line - self.focal_point.y) / 2.0
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
pub fn calc_voronoi_lines(width: f64, height: f64, points: &Vec<Point>) -> Vec<Polygon> {
    let mut points = points.clone();
    if points.len() <= 1 {
        return create_point_one_voronoi(width, height);
    } else if points.len() <= 2 {
        return create_point_twe_voronoi(width, height, &points);
    }
    points.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    // サイトイベントを登録
    let mut queue: VecDeque<Event> = VecDeque::new();
    for i in (0..points.len()).rev() {
        queue.push_back(Event {
            site: Some(points[i].clone()),
            event_type: EventType::Site,
        })
    }

    let mut base_line_y: f64 = f64::MAX;
    let beach_line: Vec<Arc> = Vec::new();
    while let Some(event) = queue.pop_front() {
        match (event.event_type, event.site) {
            (EventType::Site, Some(site)) => {
                eprintln!("site event: {:?}", site);
                base_line_y = site.y;
                eprintln!("  base line: {}", base_line_y);

                // 汀線に焦点を追加して並べ替え
                beach_line.push(Arc {
                    focal_point: site.clone(),
                });
                beach_line.sort_by(|a, b| a.focal_point.x.partial_cmp(&b.focal_point.x));

                // 基準線の更新
            }
            (EventType::Circle, _) => {}
            _ => {}
        }
    }
    vec![]
}

fn create_point_one_voronoi(width: f64, height: f64) -> Vec<Polygon> {
    let lines = create_rect_lines(&vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: width, y: 0.0 },
        Point {
            x: width,
            y: height,
        },
        Point { x: 0.0, y: height },
    ]);
    vec![Polygon { lines: lines }]
}

fn create_point_twe_voronoi(width: f64, height: f64, points: &Vec<Point>) -> Vec<Polygon> {
    let mut points = points.clone();
    points.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let delta_y = get_delta(points[1].y, points[0].y);
    let delta_x = get_delta(points[1].x, points[0].x);
    let split_line = if delta_y == 0.0 {
        eprintln!("縦線で分割");
        // 縦線
        let x = delta_x / 2.0 + min_f64(points[0].x, points[1].x);
        Line {
            p1: Point { x: x, y: 0.0 },
            p2: Point { x: x, y: height },
        }
    } else if delta_x == 0.0 {
        eprintln!("横線で分割");
        // 横線
        let y = delta_y / 2.0 + min_f64(points[0].y, points[1].y);
        Line {
            p1: Point { x: 0.0, y: y },
            p2: Point { x: width, y: y },
        }
    } else {
        let center_y = delta_y / 2.0 + points[0].y;
        let center_x = delta_x / 2.0 + points[0].x;
        eprintln!("split line center: ({}, {})", center_x, center_y);
        let a = -1.0 / (delta_y / delta_x);
        let b = center_y - a * center_x;
        eprintln!("slope: {}, section: {}", a, b);
        // 右肩上がり
        let (min_y, max_y) = if a > 0.0 {
            eprintln!("right up");
            (max_f64(0.0, b), min_f64(height, a * width + b))
        } else {
            eprintln!("right down");
            (min_f64(height, b), max_f64(0.0, a * width + b))
        };
        let max_x = (max_y - b) / a;
        let min_x = (min_y - b) / a;
        // -0.0の対応
        let (min_x, min_y, max_x, max_y) = { (min_x.abs(), min_y.abs(), max_x.abs(), max_y.abs()) };

        let min_point = Point { x: min_x, y: min_y };
        let max_point = Point { x: max_x, y: max_y };
        eprintln!("min point: {:?}", min_point);
        eprintln!("max point: {:?}", max_point);
        Line {
            p1: min_point,
            p2: max_point,
        }
    };
    eprintln!("split line: {:?}", &split_line);
    let min_point = split_line.p1.clone();
    let max_point = split_line.p2.clone();
    let left_top = Point { x: 0.0, y: height };
    let left_bottom = Point { x: 0.0, y: 0.0 };
    let right_top = Point {
        x: width,
        y: height,
    };
    let right_bottom = Point { x: width, y: 0.0 };
    let all_points = vec![left_top, left_bottom, right_top, right_bottom];

    let create_polygon_line =
        |min_point: &Point, max_point: &Point, use_points: &Vec<&Point>| -> Vec<Line> {
            eprintln!("create_polygon_line: {:?}", use_points);
            let mut indexes: Vec<usize> = Vec::new();
            // max_pointからの距離が近い準に並べる
            let mut x = max_point.x;
            let mut y = max_point.y;
            for i in 0..use_points.len() {
                let mut min_distance = f64::MAX;
                let mut min_distance_index = 0usize;
                for j in 0..use_points.len() {
                    if indexes.iter().any(|value| *value == j) {
                        continue;
                    }

                    let distance = get_distance(use_points[j].x, x, use_points[j].y, y);
                    if distance < min_distance {
                        min_distance = distance;
                        min_distance_index = j;
                    }
                }
                x = use_points[min_distance_index].x;
                y = use_points[min_distance_index].y;
                indexes.push(min_distance_index);
            }

            // min -> maxのラインが初期ライン
            let mut lines: Vec<Line> = Vec::new();
            lines.push(Line {
                p1: min_point.clone(),
                p2: max_point.clone(),
            });
            let mut x = max_point.x;
            let mut y = max_point.y;

            for i in 0..indexes.len() {
                let index = indexes[i];
                let point = use_points[index];
                if x == point.x && y == point.y {
                    continue;
                }

                lines.push(Line {
                    p1: Point { x: x, y: y },
                    p2: point.clone(),
                });
                x = point.x;
                y = point.y;
            }

            // 最終ポイントからminへのラインを追加
            if x != min_point.x || y != min_point.y {
                lines.push(Line {
                    p1: Point { x: x, y: y },
                    p2: min_point.clone(),
                });
            }

            lines
        };

    // (min_x, min_y) -> (max_x, max_y)の左回り
    // 線分から右側(上)だけを抽出して構築
    let lines1 = create_polygon_line(
        &min_point,
        &max_point,
        &all_points
            .iter()
            .filter(|point| split_line.get_point_direction(point) != LinePointDirection::Left)
            .collect::<Vec<&Point>>(),
    );

    // (max_x, max_y) -> (min_x, min_y)の右回り
    // 線分から左側(下)だけを抽出して構築
    let lines2 = create_polygon_line(
        &min_point,
        &max_point,
        &all_points
            .iter()
            .filter(|point| split_line.get_point_direction(point) != LinePointDirection::Right)
            .collect::<Vec<&Point>>(),
    );

    vec![Polygon { lines: lines1 }, Polygon { lines: lines2 }]
}

fn create_triangle_lines(points: &Vec<Point>) -> Vec<Line> {
    vec![
        Line {
            p1: points[0].clone(),
            p2: points[1].clone(),
        },
        Line {
            p1: points[1].clone(),
            p2: points[2].clone(),
        },
        Line {
            p1: points[2].clone(),
            p2: points[0].clone(),
        },
    ]
}

fn create_rect_lines(points: &Vec<Point>) -> Vec<Line> {
    vec![
        Line {
            p1: points[0].clone(),
            p2: points[1].clone(),
        },
        Line {
            p1: points[1].clone(),
            p2: points[2].clone(),
        },
        Line {
            p1: points[2].clone(),
            p2: points[3].clone(),
        },
        Line {
            p1: points[3].clone(),
            p2: points[0].clone(),
        },
    ]
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

fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x_delta = get_delta(x1, x2);
    let y_delta = get_delta(y1, y2);
    ((x_delta * x_delta) + (y_delta * y_delta)).sqrt()
}

fn get_delta(a: f64, b: f64) -> f64 {
    a - b
}

/**
 * 線分と交点を元にSVG文字列を生成
 * see: https://zenn.dev/tipstar0125/articles/d2cf0ef63bceb7
 */
pub fn create_svg(width: f64, height: f64, points: &Vec<Point>, polygons: &Vec<Polygon>) -> String {
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

    let scale_width = (580.0 / width).floor();
    let scale_height = (580.0 / width).floor();
    let change_coordinate =
        |x: f64, y: f64, scale_width: f64, scale_height: f64, margin: usize| -> (usize, usize) {
            (
                (x * scale_width) as usize + margin,
                (y * scale_height) as usize + margin,
            )
        };

    let graph_margin = margin as usize;
    for i in 0..polygons.len() {
        let polygon = &polygons[i];
        for j in 0..polygon.lines.len() {
            // x: margin + (x * scale).floor()
            // y: margin + (y * scale).floor()
            let line = &polygon.lines[j];
            let (x1, y1) = change_coordinate(
                line.p1.x,
                line.p1.y,
                scale_width,
                scale_height,
                graph_margin,
            );
            let (x2, y2) = change_coordinate(
                line.p2.x,
                line.p2.y,
                scale_width,
                scale_height,
                graph_margin,
            );
            svg = svg.add(get_svg_line(x1, y1, x2, y2, line_color));
        }
    }
    for i in 0..points.len() {
        let point = &points[i];
        let (x, y) = change_coordinate(point.x, point.y, scale_width, scale_height, graph_margin);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_voronoi_lines_point_one() {
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 50.0, y: 50.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let expect = vec![Polygon {
            lines: vec![
                Line {
                    p1: Point { x: 0.0, y: 0.0 },
                    p2: Point { x: width, y: 0.0 },
                },
                Line {
                    p1: Point { x: width, y: 0.0 },
                    p2: Point {
                        x: width,
                        y: height,
                    },
                },
                Line {
                    p1: Point {
                        x: width,
                        y: height,
                    },
                    p2: Point { x: 0.0, y: height },
                },
                Line {
                    p1: Point { x: 0.0, y: height },
                    p2: Point { x: 0.0, y: 0.0 },
                },
            ],
        }];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_horizontal_split() {
        // 横線のテスト
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 50.0, y: 10.0 }, Point { x: 50.0, y: 30.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let half_y = 20.0;
        let expect = vec![
            Polygon {
                lines: vec![
                    Line {
                        p1: Point { x: 0.0, y: half_y },
                        p2: Point {
                            x: width,
                            y: half_y,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: half_y,
                        },
                        p2: Point {
                            x: width,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                        },
                        p2: Point { x: 0.0, y: height },
                    },
                    Line {
                        p1: Point { x: 0.0, y: height },
                        p2: Point { x: 0.0, y: half_y },
                    },
                ],
            },
            Polygon {
                lines: vec![
                    Line {
                        p1: Point { x: 0.0, y: half_y },
                        p2: Point {
                            x: width,
                            y: half_y,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: half_y,
                        },
                        p2: Point { x: width, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: width, y: 0.0 },
                        p2: Point { x: 0.0, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: 0.0, y: 0.0 },
                        p2: Point { x: 0.0, y: half_y },
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_vertical_split() {
        // 縦線のテスト
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 10.0, y: 20.0 }, Point { x: 30.0, y: 20.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let half_x = 20.0;
        let expect = vec![
            Polygon {
                lines: vec![
                    Line {
                        p1: Point { x: half_x, y: 0.0 },
                        p2: Point {
                            x: half_x,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: half_x,
                            y: height,
                        },
                        p2: Point { x: 0.0, y: height },
                    },
                    Line {
                        p1: Point { x: 0.0, y: height },
                        p2: Point { x: 0.0, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: 0.0, y: 0.0 },
                        p2: Point { x: half_x, y: 0.0 },
                    },
                ],
            },
            Polygon {
                lines: vec![
                    Line {
                        p1: Point { x: half_x, y: 0.0 },
                        p2: Point {
                            x: half_x,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: half_x,
                            y: height,
                        },
                        p2: Point {
                            x: width,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                        },
                        p2: Point { x: width, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: width, y: 0.0 },
                        p2: Point { x: half_x, y: 0.0 },
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_split_up() {
        // 45°斜めの分割(右肩下がり)
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 25.0, y: 25.0 }, Point { x: 75.0, y: 75.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point { x: 0.0, y: height };
        let max_point = Point { x: width, y: 0.0 };
        let expect = vec![
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point { x: 0.0, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: 0.0, y: 0.0 },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_split_down() {
        // 45°斜めの分割(右肩上がり)
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 25.0, y: 75.0 }, Point { x: 75.0, y: 25.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point { x: 0.0, y: 0.0 };
        let max_point = Point {
            x: width,
            y: height,
        };
        let expect = vec![
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point { x: 0.0, y: height },
                    },
                    Line {
                        p1: Point { x: 0.0, y: height },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point { x: width, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: width, y: 0.0 },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_right_down_01() {
        // 斜めの分割(右上・右肩下がり)
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 70.0, y: 80.0 }, Point { x: 90.0, y: 90.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point { x: 72.5, y: height };
        let max_point = Point { x: width, y: 45.0 };
        let expect = vec![
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point { x: width, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: width, y: 0.0 },
                        p2: Point { x: 0.0, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: 0.0, y: 0.0 },
                        p2: Point { x: 0.0, y: height },
                    },
                    Line {
                        p1: Point { x: 0.0, y: height },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_right_down_02() {
        // 斜めの分割(中央付近・右肩下がり・a=2.0)
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point { x: 55.0, y: 60.0 }, Point { x: 45.0, y: 40.0 }];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point { x: 0.0, y: 75.0 };
        let max_point = Point { x: width, y: 25.0 };
        let expect = vec![
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point { x: width, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: width, y: 0.0 },
                        p2: Point { x: 0.0, y: 0.0 },
                    },
                    Line {
                        p1: Point { x: 0.0, y: 0.0 },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: height,
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                        },
                        p2: Point { x: 0.0, y: height },
                    },
                    Line {
                        p1: Point { x: 0.0, y: height },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }
}
