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
     *  */
    pub fn get_cross_points(&self, b: Arc, base_line: f64) -> f64 {
        let p = self.get_v(base_line);
        // v = (h, k)
        //   = (self.point.x, self.point.y - base_line / 2 + base_line)
        // p = 焦点から放物線の最低点までの距離
        //   = self.point.y - base_line / 2
        // 4p(y - k) = (x - h)^2
        // より
        // y = (x^2 - 2xh1 + h1^2 + 4p1k1) / 4p1
        // y = (x^2 - 2xh2 + h2^2 + 4p2k2) / 4p2
        //
        // 焦点のy位置が同じ場合はx距離の半分が交差
        // 焦点のy位置が異なる場合は別途計算

        // 参考コード
        /**
        // 左から見て母点index1の放物線と母点index2の放物線が交わる交点の位置を計算する関数
        PVector getIntersection (
          ArrayList<PVector> generating_points,
          int index1, // 左の母点
          int index2, // 右の母点
          float rho // 準線の位置
        ){
          PVector intersect = new PVector(0.0,0.0);

          float x1 = (float) generating_points.get(index1).x;
          float y1 = (float) generating_points.get(index1).y;
          float x2 = (float) generating_points.get(index2).x;
          float y2 = (float) generating_points.get(index2).y;

          float a = y2 - y1;
          float b = (rho-y1)*x2-(rho-y2)*x1;
          float c = (rho-y1)*pow(x2,2)-(rho-y2)*pow(x1,2)+(y1-y2)*(rho-y1)*(rho-y2);

          if( abs(y1 - rho) < 0.001 ){
            intersect.x = x1;
            intersect.y = quadratic_func(intersect.x, x2, y2, rho);
          } else if( abs(y2 - rho) < 0.001 ){
            intersect.x = x2;
            intersect.y = quadratic_func(intersect.x, x1, y1, rho);
          } else if( abs(a) < 0.001 ){
            intersect.x = c/b/2.0;
            intersect.y = quadratic_func(intersect.x, x1, y1, rho);
          } else if ( index1 < index2 ){
            intersect.x = (b-sqrt(pow(b,2)-a*c))/a;
            intersect.y = quadratic_func(intersect.x, x1, y1, rho);
          } else {
            intersect.x = (b-sqrt(pow(b,2)-a*c))/a;
            intersect.y = quadratic_func(intersect.x, x2, y2, rho);
          }

          return intersect;

        }
                 */
        0.0
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
