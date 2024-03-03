use proconio::input;
use std::{
    collections::HashSet,
    io::{self, BufWriter, Write},
};

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

fn main_logic<W: Write>(w: &mut W, n: usize, points: Vec<(i64, i64)>) {
    let mut inner_points: HashSet<(i64, i64)> = HashSet::from([]);
    (0..points.len()).for_each(|p0| {
        (0..points.len()).for_each(|p1| {
            if p0 != p1 {
                (0..points.len()).for_each(|p2| {
                    if p0 != p2 && p1 != p2 {
                        (0..points.len()).for_each(|p3| {
                            if p0 != p3 && p1 != p3 && p2 != p3 {
                                eprintln!(
                                    "{:?}, {:?}, {:?}, {:?}",
                                    points[p0], points[p1], points[p2], points[p3]
                                );
                                if in_triangle(points[p0], points[p1], points[p2], points[p3]) {
                                    eprintln!("in_triangle: {:?}", points[p3]);
                                    inner_points.insert(points[p3]);
                                }
                            }
                        })
                    }
                })
            }
        })
    });
    points.iter().for_each(|point| {
        if !inner_points.contains(point) {
            writeln!(w, "{},{}", point.0, point.1);
        }
    })
}

/**
 * 外積の方向を返す
 * true: 時計回り、false: 反時計回り
 */
fn get_sign_cross_product(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    (a.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (a.1 - c.1) < 0
}

fn in_triangle(a: (i64, i64), b: (i64, i64), c: (i64, i64), p: (i64, i64)) -> bool {
    let s1 = get_sign_cross_product(p, a, b);
    let s2 = get_sign_cross_product(p, b, c);
    let s3 = get_sign_cross_product(p, a, c);
    (s1 == s2) && (s2 == s3)
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
}
