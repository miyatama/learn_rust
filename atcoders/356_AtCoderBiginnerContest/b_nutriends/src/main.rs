use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        m: u64,
        a_values: [u64; m],
        x_values: [[u64; n]; m],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, m, a_values, x_values);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, m: u64, a_values: Vec<u64>, x_values: Vec<Vec<u64>>) {
    let summaries = x_values
        .iter()
        .fold(vec![0u64; m as usize], |a, b| {
            a.iter()
                .zip(b.iter())
                .map(|(a_value, b_value)| a_value + b_value)
                .collect::<Vec<u64>>()
        });

    let result = summaries
        .iter()
        .zip(a_values.iter())
        .filter(|(sum, limit)| sum < limit)
        .next()
        .is_some();
    if result {
        writeln!(w, "No").unwrap();
    } else {
        writeln!(w, "Yes").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            2,
            3,
            vec![10, 20, 30],
            vec![vec![20, 0, 10], vec![0, 100, 100]],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Yes"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            2,
            4,
            vec![10, 20, 30, 40],
            vec![vec![20, 0, 10, 30], vec![0, 100, 100, 0]],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
