use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    id: u32,
    x: f64,
    y: f64,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    id: u32,
    p1: Point,
    p2: Point,
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
}

#[derive(Debug)]
pub struct TreeNode {
    pub is_leaf: Boolean,
    pub site: point,
    pub children: Vec<TreeNode>,
}
