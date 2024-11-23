use proconio::input;
use std::collections::HashSet;
use std::io::{self, BufWriter, Write};
use std::cmp::max;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, a);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, a: Vec<u64>) {
    let a1 = parse(&a);
    let a2 = parse2(&a);
    let max1 = get_max_length(&a1);
    let max2 = get_max_length(&a2);
    let max_length = max(max1, max2);
    writeln!(w, "{}", max_length * 2).unwrap();
}

fn get_max_length(a: &Vec<Option<u64>>) -> usize {
    eprintln!("{:?}", &a);
    let mut i = 0;
    let mut max_length = 0;
    loop {
        if i >= a.len() {
            break;
        }

        if a[i].is_some() {
            let mut length = 0;
            let mut hs: HashSet<u64> = HashSet::new();
            hs.insert(a[i].unwrap());
            loop {
                length += 1;
                if i + length >= a.len() {
                    break;
                }
                if a[i + length].is_none() {
                    break;
                }
                let value = a[i + length].unwrap();
                if hs.contains(&value) {
                    break;
                }
                hs.insert(value);
            }
            if max_length < length {
                max_length = length;
            }
        }
        i += 1;
    }
    max_length
}

fn parse(a: &Vec<u64>) -> Vec<Option<u64>> {
    if a.len() % 2 == 0 {
        a.chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    } else {
        a[..a.len() - 1]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    }
}

fn parse2(a: &Vec<u64>) -> Vec<Option<u64>> {
    if a.len() % 2 == 0 {
        a[1..a.len() - 1]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    } else {
        a[1..]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 8, vec![2, 3, 1, 1, 2, 2, 1, 1]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["4"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        // 入力自体は奇数もある模様
        main_logic(&mut buff, 3, vec![1, 2, 2]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, vec![1]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, vec![1, 2, 2]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
