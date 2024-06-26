use super::common::Point;
use std::collections::HashMap;

/**
 * 汀線
 */
#[derive(Debug, PartialEq)]
pub struct BeachLine {
    pub base_y: f64,
    pub factors: Vec<BeachLineFactor>,
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

    /**
     * 放物線の交差を計算
     * 参考: https://www.mathartroom.com/processing/voronoi_diagram/
     */
    pub fn get_cross_points(&self, b: Arc, sub_line: f64) -> Point {
        let quadratic_func = |x: f64, point: &Point, sub_line: f64| -> f64 {
            -(x - point.x).powf(2.0) / 2.0 / (sub_line - point.y) + (sub_line + point.y) / 2.0
        };
        let focus1 = self.focal_point.clone();
        let focus2 = b.focal_point.clone();

        let use_self = focus1.x < focus2.x || (focus1.x == focus2.x && focus1.y > focus2.y);
        let a = focus2.y - focus1.y;
        let b = (sub_line - focus1.y) * focus2.x - (sub_line - focus2.y) * focus1.x;
        let c = (sub_line - focus1.y) * focus2.x.powf(2.0)
            - (sub_line - focus2.y) * focus1.x.powf(2.0)
            + (focus1.y - focus2.y) * (sub_line - focus1.y) * (sub_line - focus2.y);

        return if (focus1.y - sub_line).abs() < 0.001 {
            Point {
                x: focus1.x,
                y: quadratic_func(focus1.x, &focus2, sub_line),
                ..Default::default()
            }
        } else if (focus2.y - sub_line).abs() < 0.001 {
            Point {
                x: focus2.x,
                y: quadratic_func(focus2.x, &focus1, sub_line),
                ..Default::default()
            }
        } else if a.abs() < 0.001 {
            let x = c / b / 2.0;
            Point {
                x: x,
                y: quadratic_func(x, &focus1, sub_line),
                ..Default::default()
            }
        } else if use_self {
            let x = (b - (b.powf(2.0) - a * c).sqrt()) / a;
            Point {
                x: x,
                y: quadratic_func(x, &focus1, sub_line),
                ..Default::default()
            }
        } else {
            let x = (b - (b.powf(2.0) - a * c).sqrt()) / a;
            Point {
                x: x,
                y: quadratic_func(x, &focus2, sub_line),
                ..Default::default()
            }
        };
    }

    /**
     * 放物線のx範囲
     * x_min: 最小x範囲
     * y_min: 最小y範囲(準線)
     * x_max: 最大x範囲
     * y_max: 最大y範囲
     */
    pub fn get_x_range(&self, x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> (f64, f64) {
        // 焦点のy位置でx範囲を計算する
        // v = (h, k)
        //   = (self.point.x, self.point.y - y_min / 2 + y_min)
        // p = 焦点から放物線の最低点までの距離
        //   = self.point.y - y_min / 2
        // 4p(y - k) = (x - h)^2 より
        // x = sqrt(4p(y - k)) + h
        let p = self.get_v(y_min);
        let (h, k) = (self.focal_point.x, self.focal_point.y - y_min / 2.0 + y_min);
        let x_range = (4.0 * p * (y_max - k)).sqrt();
        let min = h - x_range;
        let min = if min < x_min { x_min } else { min };
        let max = x_range + h;
        let max = if max > x_max { x_max } else { max };
        (min, max)
    }
}

/**
 * 汀線の成分
 * 各焦点のIDを保持する
 */
#[derive(Debug, PartialEq)]
pub enum BeachLineFactor {
    Arc(u32),
    CrossPoint(u32, u32),
}

pub fn create_beach_line(base_y: f64, points: &Vec<Point>, width: f64) -> BeachLine {
    let mut factors: Vec<BeachLineFactor> = Vec::new();
    let pointIdIndex: HashMap<u32, usize> = points
        .iter()
        .enumerate()
        .map(|(index, point)| (point.id, index))
        .collect::<HashMap<_, _>>();
    let max_y = points
        .iter()
        .map(|point| point.y)
        .fold(f64::MIN, |m, v| m.max(v));
    for i in 0..points.len() {
        let point = points[i].clone();
        let arc = Arc {
            focal_point: point.clone(),
        };
        let (min_x, max_x) = arc.get_x_range(0.0, base_y, width, max_y);
        let v = arc.get_v(base_y);
        let mut assign = true;
        for j in 0..factors.len() {
            match factors[j] {
                BeachLineFactor::Arc(id) => {
                    let arc1 = Arc {
                        focal_point: points[*pointIdIndex.get(&id).unwrap()].clone(),
                    };
                    let (x1, x2) = arc1.get_x_range(0.0, base_y, width, max_y);
                    let v1 = arc1.get_v(base_y);
                    // 完全に包む放物線が存在する場合
                    if x1 <= min_x && x2 >= max_x && v <= v1 {
                        assign = false;
                    }
                }
                BeachLineFactor::CrossPoint(_, _) => {}
            }
        }
        if !assign {
            continue;
        }
        // 交点を持つ要素を抽出
        let mut cross_ids: Vec<usize> = Vec::new();
        for j in 0..factors.len() {}
        factors.push(BeachLineFactor::Arc(point.id));
    }
    let arcs = points
        .iter()
        .map(|point| BeachLineFactor::Arc(point.id))
        .collect::<Vec<BeachLineFactor>>();

    BeachLine {
        base_y: base_y,
        factors: factors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_beach_line_01() {
        // 焦点が1点のみ
        let width = 100.0;
        let base_y = 20.0;
        let points = vec![Point {
            id: 1,
            x: 50.0,
            y: 30.0,
            ..Default::default()
        }];
        let actual = create_beach_line(base_y, &points, width);
        let expect = BeachLine {
            base_y: base_y,
            factors: vec![BeachLineFactor::Arc(1)],
        };
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_create_beach_line_02() {
        // 焦点が縦に並んでいて2点で交わる
        let width = 100.0;
        let base_y = 0.0;
        let points = vec![
            Point {
                id: 1,
                x: 50.0,
                y: 50.0,
                ..Default::default()
            },
            Point {
                id: 2,
                x: 50.0,
                y: 20.0,
                ..Default::default()
            },
        ];
        let actual = create_beach_line(base_y, &points, width);
        let expect = BeachLine {
            base_y: base_y,
            factors: vec![
                BeachLineFactor::Arc(1),
                BeachLineFactor::CrossPoint(1, 2),
                BeachLineFactor::Arc(2),
                BeachLineFactor::CrossPoint(2, 1),
                BeachLineFactor::Arc(1),
            ],
        };
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_create_beach_line_03() {
        // 焦点のx距離が離れていて交点が存在しない
        let width = 100.0;
        let base_y = 0.0;
        let points = vec![
            Point {
                id: 1,
                x: 10.0,
                y: 20.0,
                ..Default::default()
            },
            Point {
                id: 2,
                x: 90.0,
                y: 20.0,
                ..Default::default()
            },
        ];
        let actual = create_beach_line(base_y, &points, width);
        let expect = BeachLine {
            base_y: base_y,
            factors: vec![BeachLineFactor::Arc(1), BeachLineFactor::Arc(2)],
        };
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_create_beach_line_04() {
        // 焦点が2点縦で並んでいるが交点なし
        let width = 100.0;
        let base_y = 0.0;
        let points = vec![
            Point {
                id: 1,
                x: 50.0,
                y: 20.0,
                ..Default::default()
            },
            Point {
                id: 2,
                x: 50.0,
                y: 19.0,
                ..Default::default()
            },
        ];
        let actual = create_beach_line(base_y, &points, width);
        let expect = BeachLine {
            base_y: base_y,
            factors: vec![BeachLineFactor::Arc(2)],
        };
        assert_eq!(expect, actual);
    }
}
