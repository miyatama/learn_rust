use proconio::input;
use std::collections::HashMap;
use std::io::{self, BufWriter, Write};

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

fn main_logic<W: Write>(w: &mut W, _n: u64, a: Vec<u64>) {
    let m = 998244353;
    let mut j_sum = 0u128;
    let mut hm: HashMap<u32, u32> = HashMap::new();
    for i in 1..a.len() {
        let size = a[i].to_string().len() as u32;
        match hm.get(&size) {
            None => {
                hm.insert(size, 1);
            }
            Some(&value) => {
                hm.insert(size, value + 1);
            }
        }
        j_sum += a[i] as u128 * i as u128;
    }

    let mut i_sum = 0u128;
    for i in 0..(a.len() - 1) {
        for (&size, &count) in hm.iter() {
            let adding = a[i] as u128 * 10u128.pow(size as u32) * count as u128;
            i_sum += adding;
        }
        let size = a[i + 1].to_string().len() as u32;
        let value = hm.get(&size).unwrap();
        hm.insert(size, value - 1);
    }
    let result = (i_sum as u128 + j_sum as u128) % m as u128;
    writeln!(w, "{}", result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, vec![3, 14, 15]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2044"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, vec![1001, 5, 1000000, 1000000000, 100000]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["625549048"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            100,
            vec![
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
                100_000_000,
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["947394460"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            10,
            vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["4504500"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
