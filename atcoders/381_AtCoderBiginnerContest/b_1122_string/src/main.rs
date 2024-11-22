use proconio::input;
use std::io::{self, BufWriter, Write};
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, s);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, s: String) {
    let mut hs: HashSet<char> = HashSet::new();

    if s.len() % 2 != 0 {
        writeln!(w, "No").unwrap();
        return;
    }
    for i in (0..s.len()).step_by(2) {
        let c1 = s.chars().nth(i).unwrap();
        let c2 = s.chars().nth(i + 1).unwrap();
        if c1 != c2 {
            writeln!(w, "No").unwrap();
            return;
        }
        if hs.contains(&c1) {
            writeln!(w, "No").unwrap();
            return;
        }
        hs.insert(c1);
    }

    writeln!(w, "Yes").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, "aabbcc".to_string());
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
        main_logic(&mut buff, "aab".to_string());
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
        main_logic(&mut buff, "zzzzzz".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["No"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
