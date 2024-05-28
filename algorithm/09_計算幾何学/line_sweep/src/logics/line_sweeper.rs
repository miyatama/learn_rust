use super::common::{Line, Point};
use svg::node::element::Line as SvgLine;
use svg::node::element::Rectangle as SvgRectangle;
use svg::Document;

pub fn brute_force(lines: &Vec<Line>) -> Vec<Point> {
    vec![]
}

pub fn print_line_info(lines: &Vec<Line>) {
    eprintln!("| no | start | end |");
    eprintln!("| :---- | :---- | :---- |");
    for i in 0..lines.len() {
        let line = &lines[i];
        let p1 = line.get_strat_point();
        let p2 = line.get_end_point();
        eprintln!("| {} | {:?} | {:?} |", i + 1, p1, p2);
    }
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
        eprintln!(
            "{}: {:?}, {:?} exchanged ({}, {}) to ({}, {})",
            i, line.p1, line.p2, x1, y1, x2, y2
        );
        svg = svg.add(get_svg_line(x1, y1, x2, y2, line_color));
    }

    // TODO 交点の描画

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

fn get_cross_point(l1: &Line, l2: &Line) -> Option<Point> {
    let l1_p1 = l1.get_strat_point();
    let l1_p2 = l1.get_end_point();
    let l2_p1 = l2.get_strat_point();
    let l2_p2 = l2.get_end_point();

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

        Some(Point { x: x, y: y })
    } else {
        None
    }
}

fn calc_line_factor(line: &Line) -> (f64, f64, f64) {
    let p1 = line.get_strat_point();
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
}
