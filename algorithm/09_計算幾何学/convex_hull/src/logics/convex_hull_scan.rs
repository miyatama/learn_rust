use super::common::{self, CrossProductDirection, Point};
use std::collections::HashSet;

pub fn scan(points: &Vec<Point>) -> Vec<Point> {
    if points.len() <= 3 {
        return points.clone();
    }
    let mut points = points.clone();
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut upper: Vec<usize> = Vec::new();
    upper.push(0);
    upper.push(1);
    for i in 2..points.len() {
        upper.push(i);
        let mut j = upper.len() - 1;
        // 全てが右回りであることを保証する。
        loop {
            if upper.len() <= 2 || j <= 1 {
                break;
            }
            let p1 = &points[upper[j - 2]];
            let p2 = &points[upper[j - 1]];
            let p3 = &points[upper[j]];
            let direction = common::cross_product_direction(p1, p2, p3);
            if direction == CrossProductDirection::LeftRotate {
                upper.remove(j - 1);
                j -= 1;
            } else {
                break;
            }
        }
    }

    let max_index = points.len() - 1;
    let mut lower: Vec<usize> = Vec::new();
    lower.push(max_index);
    lower.push(max_index - 1);
    for i in (0..(max_index - 1)).rev() {
        lower.push(i);
        let mut j = lower.len() - 1;
        // 全てが右回りであることを保証する。
        loop {
            if lower.len() <= 2 || j <= 1 {
                break;
            }
            let p1 = &points[lower[j - 2]];
            let p2 = &points[lower[j - 1]];
            let p3 = &points[lower[j]];
            let direction = common::cross_product_direction(p1, p2, p3);
            if direction == CrossProductDirection::LeftRotate {
                lower.remove(j - 1);
                j -= 1;
            } else {
                break;
            }
        }
    }

    upper.extend(lower);
    let uniq: HashSet<usize> = upper.into_iter().collect();
    let uniq_points = Vec::from_iter(uniq)
        .iter()
        .map(|index| points[*index].clone())
        .collect::<Vec<Point>>();
    uniq_points
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
