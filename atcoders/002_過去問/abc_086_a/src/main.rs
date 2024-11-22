use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, a, b);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, a: u64, b: u64) {
    if (a * b) % 2 == 0 {
        writeln!(w, "Even").unwrap();
    } else {
        writeln!(w, "Odd").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, 4);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Even"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, 21);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Odd"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
