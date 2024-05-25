use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, m, a, b);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: usize, m: usize, values1: Vec<u64>, values2: Vec<u64>) {
    let mut values1 = values1.clone();
    let mut values2 = values2.clone();
    values1.sort();
    values2.sort();
    let mut i = 0usize;
    let mut j = 0usize;
    let mut is_a = false;

    loop {
        if i >= n || j >= m  {
            break;
        }
        if values1[i] < values2[j] {
            if is_a {
                writeln!(w, "Yes").unwrap();
                return;
            }
            i += 1;
            is_a = true;
        } else {
            j += 1;
            is_a = false;
        }
    }

    if i < (n - 1) {
        writeln!(w, "Yes").unwrap();
    } else {
        writeln!(w, "No").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, 2, vec![3, 2, 5], vec![4, 1]);

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
        main_logic(&mut buff, 3, 2, vec![3, 1, 5], vec![4, 2]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, 1, vec![1], vec![2]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, 2, vec![1, 3, 5, 7, 9], vec![2, 4]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Yes"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic05() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 2, 5, vec![2, 4] , vec![1, 3, 5, 7, 9]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}

