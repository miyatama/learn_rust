use super::common::{self, Point, CrossProductDirection};
use std::collections::HashSet;

pub fn scan(points: &Vec<Point>) -> Vec<Point> {
    if points.len() <= 3 {
        return points.clone();
    }
    let mut points = points.clone();
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut upper: Vec<Point> = Vec::new();
    upper.push(points[0].clone());
    upper.push(points[1].clone());
    for i in 2..points.len() {
        upper.push(points[i].clone());
        let mut j = upper.len() - 1;
        // 全てが右回りであることを保証する。
        while common::cross_product_direction(&upper[j - 2], &upper[j - 1], &upper[j])
            == CrossProductDirection::LeftRotate
        {
            upper.remove(j - 1);
            j -= 1;
        }
    }

    let max_index = points.len() - 1;
    let mut lower: Vec<Point> = Vec::new();
    lower.push(points[max_index].clone());
    lower.push(points[max_index - 1].clone());
    for i in (0..(max_index - 1)).rev() {
        lower.push(points[i].clone());
        let mut j = lower.len() - 1;
        // 全てが右回りであることを保証する。
        while common::cross_product_direction(&lower[j - 2], &lower[j - 1], &lower[j])
            == CrossProductDirection::LeftRotate
        {
            lower.remove(j - 1);
            j -= 1;
        }
    }

    upper.extend(lower);
    let uniq_points: HashSet<Point> = upper.into_iter().collect();
    Vec::from_iter(uniq_points)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_scan_01() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.8, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
        ];
        let actual = scan(&points);
        assert_eq!(actual, points);
    }
}
