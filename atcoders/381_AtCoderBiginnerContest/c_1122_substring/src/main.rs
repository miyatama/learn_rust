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
    if s.find("/").is_none() {
        writeln!(w, "0").unwrap();
        return;
    }

    let mut max_length = 1;
    let mut i = 0;
    loop {
        if let Some((one_index, one_length)) = get_next_one(&s, i) {
            eprintln!("one index: {}, length: {}", one_index, one_length);
            if s.chars().nth(one_index + one_length).unwrap() == '/' {
                let two_length = get_two_length(&s, one_index + one_length + 1);
                eprintln!("two length: {}", two_length);
                let length = if one_length > two_length {
                    two_length * 2 + 1
                } else {
                    one_length * 2 + 1
                };
                if max_length < length {
                    max_length = length;
                }
                i += one_index + one_length + two_length;
            } else {
                i += one_index + one_length;
            }
        } else {
            break;
        }
    }

    writeln!(w, "{}", max_length).unwrap();
}

fn get_next_one(s: &String, index: usize) -> Option<(usize, usize)> {
    for i in index..s.len() {
        if s.chars().nth(i).unwrap() == '1' {
            for j in i..s.len() {
                if s.chars().nth(j).unwrap() != '1' {
                    return Some((i, (j - i)));
                }
            }
            return None;
        }
    }
    None
}

fn get_two_length(s: &String, index: usize) -> usize {
    for i in index..s.len() {
        if s.chars().nth(i).unwrap() != '2' {
            return i - index;
        }
    }
    s.len() - index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 8, "211/2212".to_string());

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["5"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, "22/11".to_string());

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 22, "/1211/2///2111/2222/11".to_string());

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["7"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, "11111".to_string());

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic05() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 6, "11111/".to_string());

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
