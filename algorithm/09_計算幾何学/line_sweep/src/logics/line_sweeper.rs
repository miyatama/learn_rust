use super::common::{Line, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Line as SvgLine;
use svg::node::element::Rectangle as SvgRectangle;
use svg::Document;

pub fn brute_force(lines: &Vec<Line>) -> Vec<Point> {
    let mut cross_points: Vec<Point> = Vec::new();
    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            match get_cross_point(&lines[i], &lines[j]) {
                None => {}
                Some(point) => {
                    cross_points.push(point.clone());
                }
            }
        }
    }
    cross_points
}

pub fn intersection(lines: &Vec<Line>) -> Vec<Point> {
    let mut lines = lines.clone();
    lines.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut queue: Vec<Point> = Vec::new();

    let get_queue = |point: &Point, queue: &Vec<Point>| -> Option<usize> {
        for i in 0..queue.len() {
            let queue_point = &queue[i];

            if queue_point.x == point.x && queue_point.y == point.y {
                return Some(i);
            }
        }
        None
    };

    for i in 0..lines.len() {
        let line = &lines[i];
        let points = vec![line.get_start_point(), line.get_end_point()];
        for j in 0..points.len() {
            let point = &points[j];
            match get_queue(&point, &queue) {
                None => {
                    if j == 0 {
                        queue.push(point.clone());
                    } else {
                        queue.push(point.clone());
                    }
                }
                Some(_) => {}
            }
        }
    }

    let mut queue: VecDeque<Point> = VecDeque::from(queue);
    let push_queue = |point: &Point, quque: &VecDeque<Point>| -> VecDeque<Point> {
        let mut vec = queue.into_iter().collect::<Vec<Point>>();
        vec.push(point.clone());
        vec.sort_by(|a, b| {
            a.y.partial_cmp(b.y)
                .unwrap()
                .then_with(|| self_p1.x.partial_cmp(&other_p1.x).unwrap())
        });
        VecDeque::from(vec)
    };
    // 一番上の位置に関連する線分を保持する
    let mut current_lines: Vec<Line> = Vec::new();
    let mut retain_lines: Vec<Line> = lines.clone();
    let get_retain_index = |line: &Line, retains: &Vec<Line>| -> Option<usize> {
        for i in 0..retains.len() {
            if retains[i] == line {
                return Some(i);
            }
        }
        None
    };
    let top_y = lines[0].get_start_point().y;
    current_lines.push(lines[0].clone());
    for i in 1..lines.len() {
        let line = &lines[i];
        let point = line.get_start_point();
        if top_y <= point.y {
            current_lines.push(line.clone());
if let Some(index) = get_retain_index(&line, &retain_lines) {
    retain_lines.remove(index);

}
        } else {
            break;
        }
    }
    while let Some(line_state) = queue.pop() {
        let left_neiber_line = get_left_neighbor_line(&current_lines, &line_stata.point);
        let right_neiber_line = get_right_neighbor_line(&current_lines, &line_stata.point);
        if left_neiber_line.is_some() && right_neiber_line.is_some() {
            match get_cross_point(&left_neiber_line.unwrap(), &right_neiber_line.unwrap()) {
                None => {}
                Some(point) => {
                    queue = push_queue(&point, &queue);
                }
            }
        }
        if let Some(next_line_state) = queue.front() {
        // 開始する線分を追加
        let mut remove_indexes: Vec<usize> = Vec::new();
        for i in 0..retain_lines.len() {
            let line = &retain_lines[i];
            let p1 = line.get_start_point();
            if p1.y >= next_line_state.y {
current_lines.push(line.clone());
remove_indexes.push(i);
            }
        }
        for i in 0..remove_indexes.len() {
            retain_lines.remove(remove_indexes[i]);
        }

        // 終了する線分を削除
        let mut remove_indexes: Vec<usize> = Vec::new();
        for i in 0..current_lines.len() {
            let line = &current_lines[i];
            let p2 = line.get_end_point();
            if p2.y > next_line_state.y {
remove_indexes.push(i);
            }
        }
        for i in 0..remove_indexes.len() {
            current_lines.remove(remove_indexes[i]);
        }
        }
        for i in 0..lines.len() {
            let line = &lines[i]
        }
    }
    vec![]
}

fn get_left_neighbor_line(lines: &Vec<Line>, point: &Point) -> Option<Line> {
    let mut neighbor_index: usize = 0;
    let mut distance: f64 = f64::MAX;
    for i in 0..lines.len() {
        let x = lines[i].calc_x(point.y);
        if point.x >= x && distance > (point.x - x) {
            distance = point.x - x;
            neighbor_index = i;
        }
    }

    if min_distance >= f64::MAX {
        return None;
    }
    Some(neighbor_index)
}

fn get_right_neighbor_line(lines: &Vec<Line>, point: &Point) -> Option<Line> {
    let mut neighbor_index: usize = 0;
    let mut distance: f64 = f64::MAX;
    for i in 0..lines.len() {
        let x = lines[i].calc_x(point.y);
        if point.x < x && distance > (x - point.x) {
            distance = x - point.x;
            neighbor_index = i;
        }
    }

    if min_distance >= f64::MAX {
        return None;
    }
    Some(neighbor_index)
}

pub fn print_line_info(lines: &Vec<Line>) {
    eprintln!("lines \n");
    eprintln!("| no | start | end |");
    eprintln!("| :---- | :---- | :---- |");
    for i in 0..lines.len() {
        let line = &lines[i];
        let p1 = line.get_start_point();
        let p2 = line.get_end_point();
        eprintln!(
            "| {} | ({}, {}) | ({}, {}) |",
            i + 1,
            p1.x,
            p1.y,
            p2.x,
            p2.y
        );
    }
    eprintln!("\n");
}

pub fn print_cross_point_info(points: &Vec<Point>) {
    eprintln!("cross point\n");
    eprintln!("| no | x | y |");
    eprintln!("| :---- | :---- | :---- |");
    for i in 0..points.len() {
        let point = &points[i];
        eprintln!("| {} | {} | {} |", i + 1, point.x, point.y);
    }
    eprintln!("\n");
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

    // グラフのx, y罫線を描画
    svg = svg.add(get_svg_line(300, 20, 300, 580, line_color));
    svg = svg.add(get_svg_line(20, 300, 580, 300, line_color));

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

fn get_cross_point(l1: &Line, l2: &Line) -> Option<Point> {
    let l1_p1 = l1.get_start_point();
    let l1_p2 = l1.get_end_point();
    let l2_p1 = l2.get_start_point();
    let l2_p2 = l2.get_end_point();

    // まったく同じ線分のばあいはL2の先頭ポイントを返す
    if l1_p1.x == l2_p1.x && l1_p1.y == l2_p1.y && l1_p2.x == l2_p2.x && l1_p2.y == l2_p2.y {
        return Some(l2.p1.clone());
    }

    // 2つの線分でX or Yが共通の場合はL2の先頭ポイントを返す
    if l1_p1.x == l1_p2.x && l1_p1.x == l2_p1.x && l2_p1.x == l2_p2.x {
        if l1_p2.y < l2_p1.y {
            return None;
        } else {
            return Some(l2.p1.clone());
        }
    }
    if l1_p1.y == l1_p2.y && l1_p1.y == l2_p1.y && l2_p1.y == l2_p2.y {
        if l1_p2.x < l2_p1.x {
            return None;
        } else {
            return Some(l2.p1.clone());
        }
    }

    let l1_factor = calc_line_factor(l1);
    let l2_factor = calc_line_factor(l2);

    // t1, t2: L2 * L1 factor
    let t1 = l1_factor.0 * l2_p1.x + l1_factor.1 * l2_p1.y + l1_factor.2;
    let t2 = l1_factor.0 * l2_p2.x + l1_factor.1 * l2_p2.y + l1_factor.2;
    // t3, t4: L1 * L2 factor
    let t3 = l2_factor.0 * l1_p1.x + l2_factor.1 * l1_p1.y + l2_factor.2;
    let t4 = l2_factor.0 * l1_p2.x + l2_factor.1 * l1_p2.y + l2_factor.2;
    if ((t1 >= 0.0 && t2 <= 0.0) || (t1 <= 0.0 && t2 >= 0.0))
        && ((t3 >= 0.0 && t4 <= 0.0) || (t3 <= 0.0 && t4 >= 0.0))
    {
        let x = (l1_factor.1 * l2_factor.2 - l2_factor.1 * l1_factor.2)
            / (l1_factor.0 * l2_factor.1 - l2_factor.0 * l1_factor.1);
        let y = (l2_factor.0 * l1_factor.2 - l1_factor.0 * l2_factor.2)
            / (l1_factor.0 * l2_factor.1 - l2_factor.0 * l1_factor.1);

        // TODO NANになるケースの原因調査
        if x.is_nan() || y.is_nan() {
            eprintln!("cross point calc error: l1: {:?}, l2: {:?}", l1, l2);
            eprintln!("  ts: {}, {}, {}, {}", t1, t2, t3, t4);
            eprintln!("  x: {}, y: {:?}", x, y);
            eprintln!("  l1 factor: {:?}, l2 factor: {:?}", l1_factor, l2_factor);
            return None;
        }
        Some(Point { x: x, y: y })
    } else {
        None
    }
}

fn calc_line_factor(line: &Line) -> (f64, f64, f64) {
    let p1 = line.get_start_point();
    let p2 = line.get_end_point();
    let a = p2.y - p1.y;
    let b = p1.x - p2.x;

    let c = p1.y * (p2.x - p1.x) - p1.x * (p2.y - p1.y);
    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cross_point_01() {
        let l1 = Line {
            p1: Point { x: 100.0, y: 100.0 },
            p2: Point { x: 100.0, y: 0.0 },
        };
        let l2 = Line {
            p1: Point { x: 50.0, y: 50.0 },
            p2: Point { x: 150.0, y: 50.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: 100.0, y: 50.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_02() {
        let l1 = Line {
            p1: Point { x: 100.0, y: 100.0 },
            p2: Point { x: 100.0, y: 0.0 },
        };
        let l2 = Line {
            p1: Point { x: 50.0, y: 100.1 },
            p2: Point { x: 150.0, y: 100.1 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = None;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_03() {
        let l1 = Line {
            p1: Point { x: 100.0, y: 100.0 },
            p2: Point { x: 100.0, y: 0.0 },
        };
        let l2 = Line {
            p1: Point { x: 50.0, y: 100.0 },
            p2: Point { x: 150.0, y: 100.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: 100.0, y: 100.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_04() {
        let l1 = Line {
            p1: Point { x: 100.0, y: 100.0 },
            p2: Point { x: 100.0, y: 0.0 },
        };
        let l2 = Line {
            p1: Point { x: 50.0, y: 0.0 },
            p2: Point { x: 150.0, y: 0.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: 100.0, y: 0.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_05() {
        let l1 = Line {
            p1: Point { x: 50.0, y: 100.0 },
            p2: Point { x: 50.0, y: 0.0 },
        };
        let l2 = Line {
            p1: Point { x: 50.0, y: 50.0 },
            p2: Point { x: 150.0, y: 50.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: 50.0, y: 50.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_06() {
        let l1 = Line {
            p1: Point { x: 150.0, y: 100.0 },
            p2: Point { x: 150.0, y: 0.0 },
        };
        let l2 = Line {
            p1: Point { x: 50.0, y: 50.0 },
            p2: Point { x: 150.0, y: 50.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: 150.0, y: 50.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_07() {
        let l1 = Line {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point { x: 10.0, y: 10.0 },
        };
        let l2 = Line {
            p1: Point { x: 8.0, y: 1.0 },
            p2: Point { x: 9.0, y: 0.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = None;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_08() {
        let l1 = Line {
            p1: Point { x: -3.0, y: 4.0 },
            p2: Point { x: -3.0, y: -3.0 },
        };
        let l2 = Line {
            p1: Point { x: -3.0, y: 3.0 },
            p2: Point { x: -3.0, y: -5.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: -3.0, y: 3.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_09() {
        let l1 = Line {
            p1: Point { x: -1.0, y: 2.0 },
            p2: Point { x: 2.0, y: 2.0 },
        };
        let l2 = Line {
            p1: Point { x: 0.0, y: 2.0 },
            p2: Point { x: 5.0, y: 2.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: 0.0, y: 2.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_10() {
        let l1 = Line {
            p1: Point { x: -6.0, y: 4.0 },
            p2: Point { x: 1.0, y: 4.0 },
        };
        let l2 = Line {
            p1: Point { x: 2.0, y: 4.0 },
            p2: Point { x: 4.0, y: 4.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = None;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_11() {
        let l1 = Line {
            p1: Point { x: -2.0, y: -2.0 },
            p2: Point { x: -2.0, y: -4.0 },
        };
        let l2 = Line {
            p1: Point { x: -2.0, y: 3.0 },
            p2: Point { x: -2.0, y: 1.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = None;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_12() {
        let l1 = Line {
            p1: Point { x: -2.0, y: -2.0 },
            p2: Point { x: 5.0, y: -4.0 },
        };
        let l2 = Line {
            p1: Point { x: -2.0, y: -2.0 },
            p2: Point { x: 5.0, y: -4.0 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: -2.0, y: -2.0 });
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_cross_point_13() {
        let l1 = Line {
            p1: Point { x: -2.0, y: -2.0 },
            p2: Point { x: -2.0, y: -4.0 },
        };
        let l2 = Line {
            p1: Point { x: -2.0, y: -2.5 },
            p2: Point { x: -2.0, y: -3.5 },
        };
        let actual = get_cross_point(&l1, &l2);
        let expect = Some(Point { x: -2.0, y: -2.5 });
        assert_eq!(expect, actual);
    }
}
