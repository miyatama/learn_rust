use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum CrossProductDirection {
    LeftRotate,
    RightRotate,
    Equally,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.x
                .partial_cmp(&other.x)
                .unwrap()
                .then_with(|| self.y.partial_cmp(&other.y).unwrap()),
        )
    }
}

/**
 * 対称の座標が三角形内に存在するかを判定する
 */
pub fn in_triangle(p1: &Point, p2: &Point, p3: &Point, t: &Point) -> bool {
    let cp1 = cross_product_direction(p1, p2, t);
    let cp2 = cross_product_direction(p3, p1, t);
    let cp3 = cross_product_direction(p2, p3, t);
    cp1 == cp2 && cp2 == cp3
}

pub fn print_points(title: String, points: &Vec<Point>) {
    let max_distance = vec![
        points
            .iter()
            .map(|point| point.x.abs())
            .fold(0.0f64, |m, v| m.max(v))
            .abs(),
        points
            .iter()
            .map(|point| point.y.abs())
            .fold(0.0f64, |m, v| m.max(v))
            .abs(),
    ]
    .iter()
    .fold(0.0f64, |m, v| m.max(*v));

    // 中心を0.5, 0.5としてプロットする
    // 全体範囲は[0-1, 0-1]

    println!("```mermaid");
    println!("quadrantChart");
    println!("title {}", title);
    for i in 0..points.len() {
        println!(
            "{}: [{}, {}]",
            i,
            points[i].x / max_distance * 0.5 + 0.5,
            points[i].y / max_distance * 0.5 + 0.5,
        );
    }
    println!("```")
}

pub fn cross_product_direction(p1: &Point, p2: &Point, t: &Point) -> CrossProductDirection {
    match (p2.x - p1.x) * (t.y - p1.y) - (p2.y - p1.y) * (t.x - p1.x) {
        cp if cp < 0.0 => CrossProductDirection::RightRotate,
        cp if cp > 0.0 => CrossProductDirection::LeftRotate,
        _ => CrossProductDirection::Equally,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_sort_01() {
        let mut points = vec![
            Point { x: 100.0, y: 100.0 },
            Point { x: 100.0, y: 80.0 },
            Point { x: 20.0, y: 20.0 },
            Point { x: 20.0, y: 10.0 },
            Point { x: -5.0, y: -5.0 },
            Point { x: -5.0, y: -3.0 },
        ];
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let expect = vec![
            Point { x: -5.0, y: -5.0 },
            Point { x: -5.0, y: -3.0 },
            Point { x: 20.0, y: 10.0 },
            Point { x: 20.0, y: 20.0 },
            Point { x: 100.0, y: 80.0 },
            Point { x: 100.0, y: 100.0 },
        ];
        assert_eq!(expect, points);
    }

    #[test]
    fn test_in_triangle_01() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.5, y: 2.0 },
        ];
        let actual = in_triangle(&points[0], &points[1], &points[2], &points[3]);
        let expect = false;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_in_triangle_02() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 1.1, y: 0.0 },
        ];
        let actual = in_triangle(&points[0], &points[1], &points[2], &points[3]);
        let expect = false;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_in_triangle_03() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.2, y: -0.2 },
        ];
        let actual = in_triangle(&points[0], &points[1], &points[2], &points[3]);
        let expect = false;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_in_triangle_04() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.5, y: 0.5 },
        ];
        let actual = in_triangle(&points[0], &points[1], &points[2], &points[3]);
        let expect = true;
        assert_eq!(expect, actual);
    }
    #[test]
    fn test_cross_product_direction_01() {
        let points = vec![
            Point { x: 1.0, y: 2.0 },
            Point { x: 1.0, y: 2.0 },
            Point { x: 1.0, y: 2.0 },
        ];
        let actual = cross_product_direction(&points[0], &points[1], &points[2]);
        let expect = CrossProductDirection::Equally;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_cross_product_direction_02() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
        ];
        let actual = cross_product_direction(&points[0], &points[1], &points[2]);
        let expect = CrossProductDirection::RightRotate;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_cross_product_direction_03() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 0.5, y: 2.0 },
        ];
        let actual = cross_product_direction(&points[0], &points[1], &points[2]);
        let expect = CrossProductDirection::LeftRotate;
        assert_eq!(expect, actual);
    }
}
