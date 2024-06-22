use super::common::Point;

/**
 * 汀線
 */
#[derive(Debug, PartialEq)]
pub struct BeachLine {
    pub base_y: f64,
    pub factors: Vec<BeachLineFactor>,
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
    let max_y = points.iter().map(|point| point.y).fold(f64::MIN, |m, v| m.max(v));
    let arcs = points.iter().map(|point| BeachLineFactor::Arc(point.id)).collect::<Vec<BeachLineFactor>>();
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
            factors: vec![
                BeachLineFactor::Arc(1),
                BeachLineFactor::Arc(2),
            ],
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
            factors: vec![
                BeachLineFactor::Arc(2),
            ],
        };
        assert_eq!(expect, actual);
    }
}
