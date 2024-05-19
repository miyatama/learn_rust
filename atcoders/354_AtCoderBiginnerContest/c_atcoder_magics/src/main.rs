use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        cards: [(u64, u64); n]
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, cards);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, _n: u64, cards: Vec<(u64, u64)>) {
    let mut cards = cards
        .iter()
        .enumerate()
        .map(|(index, (power, cost))| (*power, (*cost, index + 1)))
        .collect::<Vec<(u64, (u64, usize))>>()
        .clone();
    cards.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());
    let mut retain_cards: Vec<(u64, (u64, usize))> = Vec::new();

    retain_cards.push(cards[cards.len() - 1].clone());
    for i in (0..cards.len() - 1).rev() {
        if cards[i].1 .0 < retain_cards[retain_cards.len() - 1].1 .0 {
            retain_cards.push(cards[i].clone());
        }
    }
    let mut retain_numbers = retain_cards
        .iter()
        .map(|(_, (_, index))| *index)
        .collect::<Vec<usize>>();
    retain_numbers.sort();

    writeln!(w, "{}", retain_numbers.len()).unwrap();
    writeln!(
        w,
        "{}",
        retain_numbers
            .iter()
            .map(|number| number.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, vec![(2, 4), (1, 1), (3, 2)]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2", "2 3"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            5,
            vec![(1, 1), (10, 2), (100, 3), (1000, 4), (10000, 5)],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["5", "1 2 3 4 5"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            6,
            vec![(32, 101), (65, 78), (2, 29), (46, 55), (103, 130), (52, 40)],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["4", "2 3 5 6"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            5,
            vec![(1, 10), (2, 55), (3, 50), (4, 110), (5, 100)],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["3", "1 3 5"];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
