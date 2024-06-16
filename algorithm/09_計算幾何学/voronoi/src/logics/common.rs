use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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
    pub lines: Vec<Line>,
}
