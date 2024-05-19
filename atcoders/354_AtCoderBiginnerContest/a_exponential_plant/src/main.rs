use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        h: u64,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, h);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, h: u64) {
    let mut grass = 1;
    let mut i = 0u64;
    while grass <= h {
        i += 1;
        let delta = calc_pow(2, i);
        grass += delta;
    }
    writeln!(w, "{}", i + 1).unwrap();
}

fn calc_pow(base: u64, n: u64) -> u64 {
    if n <= 0 {
        return 1;
    }
    base * calc_pow(base, n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 54);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["6"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 7);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["4"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 262144);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["19"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic05() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 100000000);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["27"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }


    #[test]
    fn test_calc_pow_01() {
        let actual = calc_pow(2, 0);
        assert_eq!(1, actual);
    }

    #[test]
    fn test_calc_pow_02() {
        let actual = calc_pow(2, 1);
        assert_eq!(2, actual);
    }

    #[test]
    fn test_calc_pow_03() {
        let actual = calc_pow(2, 2);
        assert_eq!(4, actual);
    }
}
