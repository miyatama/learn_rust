use proconio::input;
use std::{
    collections::HashSet,
    io::{self, BufWriter, Write},
};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, points);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, _n: usize, default_points: Vec<(i64, i64)>) {
    let points = default_points
        .into_iter()
        .map(|point| Point {
            x: point.0,
            y: point.1,
        })
        .collect::<Vec<Point>>();
    let inner_points =
        generate_inner_point(&points, vec![], &HashSet::new()).unwrap_or(HashSet::new());
    points.iter().for_each(|point| {
        if !inner_points.contains(point) {
            writeln!(w, "{},{}", point.x, point.y).unwrap();
        }
    })
}

fn generate_inner_point(
    points: &Vec<Point>,
    triangle_points: Vec<Point>,
    inner_points: &HashSet<Point>,
) -> Option<HashSet<Point>> {
    if triangle_points.len() < 4 {
        let mut hs: HashSet<Point> = HashSet::from_iter(
            points
                .iter()
                .filter(|val| !inner_points.contains(val))
                .map(|val| {
                    let mut new_triangle_points = triangle_points.to_vec();
                    let mut new_points = points.to_vec();
                    let position = points.iter().position(|v| v == val).unwrap();
                    new_points.remove(position);
                    new_triangle_points.push(*val);
                    generate_inner_point(&new_points, new_triangle_points, inner_points)
                        .unwrap_or(HashSet::new())
                        .iter()
                        .copied()
                        .collect::<Vec<_>>()
                })
                .flatten(),
        );
        hs.extend(inner_points);
        Some(hs)
    } else {
        if in_triangle(
            triangle_points[0],
            triangle_points[1],
            triangle_points[2],
            triangle_points[3],
        ) {
            let mut hs: HashSet<Point> = HashSet::new();
            hs.insert(triangle_points[3]);
            Some(hs)
        } else {
            None
        }
    }
}

fn in_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let ab = Point {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let bc = Point {
        x: c.x - b.x,
        y: c.y - b.y,
    };
    let ca = Point {
        x: a.x - c.x,
        y: a.y - c.y,
    };

    let ap = Point {
        x: p.x - a.x,
        y: p.y - a.y,
    };
    let bp = Point {
        x: p.x - b.x,
        y: p.y - b.y,
    };
    let cp = Point {
        x: p.x - c.x,
        y: p.y - c.y,
    };

    let c1 = ab.x * bp.y - ab.y * bp.x;
    let c2 = bc.x * cp.y - bc.y * cp.x;
    let c3 = ca.x * ap.y - ca.y * ap.x;

    (c1 > 0 && c2 > 0 && c3 > 0) || (c1 < 0 && c2 < 0 && c3 < 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic_01() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3usize,
            vec![(0i64, 0i64), (1i64, 0i64), (0i64, 1i64)],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual
            .split("\n")
            .filter(|it| it.len() > 0)
            .collect::<Vec<&str>>();
        let expect = vec!["0,0", "1,0", "0,1"];
        assert_eq!(expect.len(), actual.len());
        expect.into_iter().zip(actual).for_each(|(expect, actual)| {
            assert_eq!(expect, actual);
        });
    }

    #[test]
    fn test_main_logic_02() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3usize,
            vec![(0i64, 0i64), (100i64, 0i64), (0i64, 100i64), (20i64, 20i64)],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual
            .split("\n")
            .filter(|it| it.len() > 0)
            .collect::<Vec<&str>>();
        let expect = vec!["0,0", "100,0", "0,100"];
        assert_eq!(expect.len(), actual.len());
        expect.into_iter().zip(actual).for_each(|(expect, actual)| {
            assert_eq!(expect, actual);
        });
    }

    #[test]
    fn test_main_logic_03() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            5usize,
            vec![(1, 1), (1, 3), (3, 3), (3, 1), (2, 2)],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual
            .split("\n")
            .filter(|it| it.len() > 0)
            .collect::<Vec<&str>>();
        let expect = vec!["1,1","1,3","3,3","3,1","2,2"];
        assert_eq!(expect.len(), actual.len());
        expect.into_iter().zip(actual).for_each(|(expect, actual)| {
            assert_eq!(expect, actual);
        });
    }

    #[test]
    fn test_main_logic_04() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            14usize,
            vec![
                (1, 7),
                (2, 10),
                (3, 7),
                (4, 1),
                (5, 12),
                (6, 9),
                (6, 5),
                (7, 1),
                (9, 4),
                (8, 8),
                (7, 13),
                (10, 11),
                (11, 6),
                (12, 3),
            ],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual
            .split("\n")
            .filter(|it| it.len() > 0)
            .collect::<Vec<&str>>();
        let expect = vec!["1,7","2,10","4,1","5,12","7,1","7,13","10,11","12,3"];
        assert_eq!(expect.len(), actual.len());
        expect.into_iter().zip(actual).for_each(|(expect, actual)| {
            assert_eq!(expect, actual);
        });
    }

    #[test]
    fn test_main_logic_05() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 4usize, vec![(1, 1), (1, 3), (3, 3), (3, 1)]);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual
            .split("\n")
            .filter(|it| it.len() > 0)
            .collect::<Vec<&str>>();
        let expect = vec!["1,1","1,3","3,3","3,1"];
        assert_eq!(expect.len(), actual.len());
        expect.into_iter().zip(actual).for_each(|(expect, actual)| {
            assert_eq!(expect, actual);
        });
    }

    #[test]
    fn test_main_logic_06() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            8usize,
            vec![
                (1, 7),
                (2, 10),
                (5, 12),
                (7, 13),
                (10, 11),
                (12, 3),
                (7, 1),
                (4, 1),
            ],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual
            .split("\n")
            .filter(|it| it.len() > 0)
            .collect::<Vec<&str>>();
        let expect = vec!["1,7","2,10","5,12","7,13","10,11","12,3","7,1","4,1"];
        assert_eq!(expect.len(), actual.len());
        expect.into_iter().zip(actual).for_each(|(expect, actual)| {
            assert_eq!(expect, actual);
        });
    }
}
