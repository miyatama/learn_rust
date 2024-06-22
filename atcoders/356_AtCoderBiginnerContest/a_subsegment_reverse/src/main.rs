use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        values: [u64; 3],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, values);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, values: Vec<u64>) {
    let mut vec: Vec<u64> = (1u64..(values[1])).collect();
    let center: Vec<u64> = (values[1]..=values[2]).rev().collect();
    let tail : Vec<u64> = ((values[2] + 1)..=values[0]).collect();
    vec.extend(center);
    vec.extend(tail);
    let text = vec
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(w, "{}", text).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, vec![5, 2, 3]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1 3 2 4 5"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, vec![7, 1, 1]);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1 2 3 4 5 6 7"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, vec![10, 1, 10]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["10 9 8 7 6 5 4 3 2 1"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, vec![10, 9, 10]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1 2 3 4 5 6 7 8 10 9"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic05() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, vec![10, 1, 2]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2 1 3 4 5 6 7 8 9 10"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
