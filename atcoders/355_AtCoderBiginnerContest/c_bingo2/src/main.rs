use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        t: u64,
        a: [u64; t],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, t, a);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64, t: u64, numbers: Vec<u64>) {
    let target_numbers = numbers
        .iter()
        .enumerate()
        .filter(|(_, number)| **number >= 1 && **number <= (n * n))
        .map(|(index, number)| (index, *number))
        .collect::<Vec<(usize, u64)>>();
    let n = n as usize;
    let mut bingo: Vec<bool> = Vec::new();
    for i in 0..(n * n) {
        bingo.push(false);
    }
    for index in 0..target_numbers.len() {
        let number = target_numbers[index].1;
        let number_usize = number as usize;
        bingo[number_usize - 1] = true;

        let i = (number_usize / n) - 1;
        let j = (number_usize - 1) % n;

        // 横の確認
        let min_index = i * n;
        let max_index = (i * n) + (n - 1);
        if bingo[min_index..max_index]
            .iter()
            .filter(|value| **value == false)
            .is_none()
        {
            writeln!(w, "{}", target_numbers[index].1).unwrap();
        }

        // 縦の確認
        let min_index = j;
        let max_index = (n - 1) * n + j;
        if bingo[(min_index..=max_index).step_by(n)]
            .iter()
            .filter(|value| *value == false)
            .is_none()
        {
            writeln!(w, "{}", target_numbers[index].1).unwrap();
        }

        // 対角線の確認
        let min_index = 0;
        let max_index = n * n - 1;
        if bingo[(min_index..=max_index).step_by(n + 1)]
            .iter()
            .filter(|value| *value == false)
            .is_none()
        {
            writeln!(w, "{}", target_numbers[index].1).unwrap();
        }

        // 対角線の確認
        let min_index = n - 1;
        let max_index = n * n - 1;
        if bingo[(min_index..=max_index).step_by(n - 1)]
            .iter()
            .filter(|value| *value == false)
            .is_none()
        {
            writeln!(w, "{}", target_numbers[index].1).unwrap();
        }
    }
    writeln!(w, "-1").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, 5, vec![5, 1, 8, 9, 7]);

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
        main_logic(&mut buff, 3, 5, vec![4, 2, 9, 7, 5]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["-1"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            4,
            12,
            vec![13, 9, 6, 5, 2, 7, 16, 14, 8, 3, 10, 11],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["9"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
