use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, a, b);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, a: i64, b: i64) {
    let result = if a != b {
        let humans: Vec<i64> = vec![1, 2, 3];
        humans.into_iter().filter(|value| *value != a && *value != b).next().unwrap()
    } else {
        -1
    };
    writeln!(w, "{}", result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, 2);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["3"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, 1);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["-1"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, 1);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}