use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, a, 10u64.pow(8));
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, a: Vec<u64>, threshold: u64) {
    eprintln!("main logic: n: {}", n);
    let mut array = a.clone();
    array.sort();
    let mut factor1 = 0;
    for i in 0..array.len() {
        factor1 += array[i] * (n - 1 as u64);
        eprintln!("factor1: {} -> {}", i, factor1);
    }
    eprintln!("factor1: {} ", factor1);
    let mut kenzan  = 0;
    for i in 0..(array.len() - 1) {
        for j in (i + 1)..array.len() {
            kenzan += array[i] + array[j];
        }
    }
    eprintln!("kenzan factor1: {}", kenzan);

    let mut factor2 = 0u64;
    for i in 0..(array.len() - 1) {
        let threshold = threshold - array[i];
        let count = array[(i + 1)..]
            .iter()
            .filter(|&value| *value >= threshold)
            .count() as u64;
        eprintln!("factor2 ai: {}, threshold: {}, count: {}", array[i], threshold, count);
        factor2 += count;
    }

    eprintln!("x = {} - {} * {}", factor1, threshold, factor2);
    let factor3: u128 = threshold  as u128 * factor2 as u128;
    let result: u128 = factor1  as u128 - factor3;
    eprintln!("{} = {} - {} * {}", result, factor1, threshold, factor2);
    writeln!(w, "{}", result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, vec![3, 50000001, 50000002], 10u64.pow(8));

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["100000012"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, vec![1, 3, 99999999, 99999994, 1000000], 10u64.pow(8));

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["303999988"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 5, vec![1, 2, 3, 4, 5], 6u64);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["24"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
