use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        s: String,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, s);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, s: String) {
    if n % 2 == 0 {
        writeln!(w, "No").unwrap();
        return;
    }

    if s.chars().filter(|value| *value == '/').count() != 1 {
        writeln!(w, "No").unwrap();
        return;
    }

    let slash_position = (s.find("/").unwrap() as u64) + 1;
    if (n + 1) / 2 != slash_position {
        writeln!(w, "No").unwrap();
        return;
    }

    let v: Vec<&str> = s.split('/').collect();
    if v[0].chars().filter(|value| *value != '1').count() != 0 {
        writeln!(w, "No").unwrap();
        return;
    }
    if v[1].chars().filter(|value| *value != '2').count() != 0 {
        writeln!(w, "No").unwrap();
        return;
    }

    writeln!(w, "Yes").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, "11/22".to_string());
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
        main_logic(&mut buff, 1, "/".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Yes"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 4, "1/22".to_string());
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
        main_logic(&mut buff, 5, "22/11".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic05() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, "11/22".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Yes"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic06() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 7, "111/222".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Yes"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic07() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 4, "1122".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic08() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 4, "1/22".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic09() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 7, "11/2222".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }


    #[test]
    fn test_main_logic10() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, "22/11".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic11() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 9, "//2/2/211".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
