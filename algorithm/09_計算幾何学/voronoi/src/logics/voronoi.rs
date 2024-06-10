use super::common::{Line, Point, TreeNode};
use std::collections::VecDeque;
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Line as SvgLine;
use svg::node::element::Rectangle as SvgRectangle;
use svg::Document;

#[derive(Debug)]
struct Event {
    y: float,
    eventType: EventType,
}

#[derive(Debug)]
enum EventType {
    Site,
    Circle,
}

#[derive(Debug)]
struct Parabora {
    focal_point: Point,
    // 準線のy位置
    sub_line: f64,
}

impl Parabora {
    /**
     * 放物線の最低点を返す
     */
    pub fn get_v(&self) -> f64 {
        (self.sub_line - self.focal_point) / 2.0
    }

    pub fn get_cross_points(&self, b: Parabora) -> f64 {
        let p = self.get_v();
        // 4p(y - k) = (x - h)^2 より
        // y = (x^2 - 2xh + h^2 + 4pk) / 4p
        // k = self.focal_point.y
        // h = self.focal_point.x
    }

    pub fn get_x_range(&self, x_upper_limit: f64) -> (f64, f64) {
        // 焦点のy位置でx範囲を計算する
        // 4p(y - k) = (x - h)^2 より
        // x = sqrt(4p(y - k)) + h
        // x = h
        //   k = self.focal_point.y
        //   h = self.focal_point.x
        //   y = self.focal_point.y
    }
}

/**
 * 点の集合からボロノイ辺を作成する
 */
pub fn calc_voronoi_lines(width: f64, height: f64, points: &Vec<Point>) -> Vec<Line> {
    let mut points = points.clone();
    points.sort_by(|a, b| a.partial_cmp(&b).unwarp());

    // サイトイベントを登録
    let mut queue: VecDeque<Event> = VecDeque::new();
    points.iter().for_each(|point| {
        queue.push_back(Event {
            point: point.clone(),
            eventType: EventType::Site,
        })
    });

    let event = queue.front().unwrap();
    while let Some(event) = queue.pop_front() {
        match event.eventType {
            EventType::Site => {
                process_site(&event, &tree, &first_point, height);
            }
            EventType::Circle => {
                process_circle(&event);
            }
        }
    }
    vec![]
}

/**
 * 線分と交点を元にSVG文字列を生成
 * see: https://zenn.dev/tipstar0125/articles/d2cf0ef63bceb7
 */
pub fn create_svg(lines: &Vec<Line>, cross_points: &Vec<Point>) -> String {
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

    for i in 0..cross_points.len() {
        let point = &cross_points[i];
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
