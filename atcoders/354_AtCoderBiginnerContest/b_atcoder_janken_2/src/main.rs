use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        users: [(String, u64); n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, users);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, users: Vec<(String, u64)>) {
    let total = users
        .iter()
        .map(|(_, score)| score)
        .fold(0, |sum, value| sum + value);
    let mut usernames = users
        .iter()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<String>>()
        .clone();
    usernames.sort();
    let target_index = total % n;
    for i in 0..usernames.len() {
        let index = i as u64;
        if index == target_index {
            writeln!(w, "{}", usernames[i]).unwrap();
            break;
        }
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
            3,
            vec![
                ("takahashi".to_string(), 2),
                ("aoki".to_string(), 6),
                ("snuke".to_string(), 5),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["snuke"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3,
            vec![
                ("takahashi".to_string(), 2813),
                ("takahashixx".to_string(), 1086),
                ("takahashix".to_string(), 4229),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["takahashix"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
