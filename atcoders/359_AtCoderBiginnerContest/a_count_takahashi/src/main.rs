use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        s: [String; n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, s);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, s: Vec<String>) {
    let takahashi = "Takahashi".to_string();
    let count = s.iter().filter(|value| *value == &takahashi).count();
    writeln!(w, "{}", count).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3,
            vec![
                "Aoki".to_string(),
                "Takahashi".to_string(),
                "Takahashi".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 2, vec!["Aoki".to_string(), "Aoki".to_string()]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            20,
            vec![
                "Aoki".to_string(),
                "Takahashi".to_string(),
                "Takahashi".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Takahashi".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Takahashi".to_string(),
                "Takahashi".to_string(),
                "Aoki".to_string(),
                "Takahashi".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Aoki".to_string(),
                "Takahashi".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["7"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
