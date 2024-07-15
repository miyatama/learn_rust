use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub id: u32,
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new() -> Point {
        Point::default()
    }

    pub fn new2(x: f64, y: f64) -> Point {
        Point { id: 0, x: x, y: y }
    }

    pub fn dist(&self, point: &Point) -> f64 {
        let x = if self.x > point.x {
            self.x - point.x
        } else {
            point.x - self.x
        };
        let y = if self.y > point.y {
            self.y - point.y
        } else {
            point.y - self.y
        };
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    }
}

impl Default for Point {
    fn default() -> Point {
        Point {
            id: 0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.y
                .partial_cmp(&other.y)
                .unwrap()
                .then_with(|| self.x.partial_cmp(&other.x).unwrap()),
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum LinePointDirection {
    Left,
    Right,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl PartialOrd for Line {
    // 並び順はy降順、x昇順
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_p1 = self.get_start_point();
        let other_p1 = other.get_start_point();
        Some(
            self_p1
                .y
                .partial_cmp(&other_p1.y)
                .unwrap()
                .then_with(|| self_p1.x.partial_cmp(&other_p1.x).unwrap()),
        )
    }
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
        Line {
            p1: Point::new2(x1, y1),
            p2: Point::new2(x2, y2),
        }
    }

    // yが大きい方を優先
    pub fn get_start_point(&self) -> Point {
        match (
            self.p1.y.partial_cmp(&self.p2.y).unwrap(),
            self.p1.x.partial_cmp(&self.p2.x).unwrap(),
        ) {
            (Ordering::Less, _) => self.p1.clone(),
            (Ordering::Greater, _) => self.p2.clone(),
            (_, Ordering::Less) => self.p1.clone(),
            _ => self.p2.clone(),
        }
    }

    pub fn get_end_point(&self) -> Point {
        match (
            self.p1.y.partial_cmp(&self.p2.y).unwrap(),
            self.p1.x.partial_cmp(&self.p2.x).unwrap(),
        ) {
            (Ordering::Greater, _) => self.p1.clone(),
            (Ordering::Less, _) => self.p2.clone(),
            (_, Ordering::Greater) => self.p1.clone(),
            _ => self.p2.clone(),
        }
    }

    /**
     * ax + by + c = 0のa, b, cを算出
     */
    pub fn get_factors(&self) -> (f64, f64, f64) {
        let p1 = self.get_start_point();
        let p2 = self.get_end_point();
        let a = p2.y - p1.y;
        let b = p1.x - p2.x;

        let c = p1.y * (p2.x - p1.x) - p1.x * (p2.y - p1.y);
        (a, b, c)
    }

    pub fn calc_x(&self, y: f64) -> f64 {
        let factors = self.get_factors();
        ((factors.1 * y) * -1.0 + factors.2 * -1.0) / factors.0
    }

    pub fn get_point_direction(&self, point: &Point) -> LinePointDirection {
        let factors = self.get_factors();
        let direction = factors.0 * point.x + factors.1 * point.y + factors.2;
        if direction > 0.0 {
            LinePointDirection::Left
        } else if direction < 0.0 {
            LinePointDirection::Right
        } else {
            LinePointDirection::Equal
        }
    }
}

/**
 * 凸法の線分集合
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub point_id: u32,
    pub lines: Vec<Line>,
}

impl Polygon {
    pub fn new(point_id: u32) -> Polygon {
        Polygon {
            point_id: point_id,
            lines: vec![],
        }
    }
    pub fn print(&self) {
        eprintln!("polygon: {}", self.point_id);
        for i in 0..self.lines.len() {
            eprintln!(
                "  [{}]: ({}, {}) to ({}, {})",
                i, self.lines[i].p1.x, self.lines[i].p1.y, self.lines[i].p2.x, self.lines[i].p2.y
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_sort_01() {
        let mut points = vec![
            Point {
                id: 100,
                x: 100.0,
                y: 100.0,
            },
            Point {
                id: 101,
                x: 100.0,
                y: 50.0,
            },
            Point {
                id: 102,
                x: 50.0,
                y: 50.0,
            },
            Point {
                id: 103,
                x: 50.0,
                y: 0.0,
            },
        ];
        points.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        let actual = points.iter().map(|point| point.id).collect::<Vec<u32>>();
        // idで確認
        let expect = vec![103, 102, 101, 100];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }
}
