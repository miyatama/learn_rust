use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        k: u64,
        a: [u64; n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, k, a);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, k: u64, a: Vec<u64>) {
    let mut count = 0;
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i];
        if sum > k {
            sum = a[i];
            count += 1;
        } 
    }
    if sum >= 1 {
        count += 1;
    }
    writeln!(w, "{}", count).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 7,6, vec![2,5,1,4,1,2,3]);

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
        main_logic(&mut buff, 7,10, vec![1,10,1,10,1,10,1]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["7"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 15,100, vec![73,8,55,26,97,48,37,47,35,55,5,17,62,2,60]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["8"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 10, 10, vec![10,10,10,10,10,10,10,10,10,10]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["10"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
