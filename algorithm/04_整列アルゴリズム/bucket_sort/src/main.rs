use proconio::input;
use std::collections::HashSet;
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
    let list = bucket_sort(a);
    eprintln!("result: {:?}", list);
    list.iter().for_each(|val| {
        writeln!(w, "{}", val).unwrap();
    });
}

fn bucket_sort(src_list: Vec<u64>) -> Vec<u64> {
    let mut hs: HashSet<u64> = HashSet::new();
    src_list.iter().for_each(|val| {
        hs.insert(*val);
    });
    let mut list = Vec::from_iter(hs);
    list.sort();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 10, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            64,
            vec![
                99, 97, 94, 93, 92, 91, 90, 89, 87, 86, 84, 83, 82, 81, 80, 79, 77, 76, 72, 71, 69,
                68, 67, 65, 64, 63, 61, 59, 58, 57, 56, 55, 52, 51, 50, 46, 44, 43, 42, 41, 40, 39,
                38, 37, 35, 34, 33, 32, 30, 28, 26, 25, 24, 23, 21, 18, 17, 16, 13, 11, 7, 6, 1, 0,
            ],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec![
            "0", "1", "6", "7", "11", "13", "16", "17", "18", "21", "23", "24", "25", "26", "28",
            "30", "32", "33", "34", "35", "37", "38", "39", "40", "41", "42", "43", "44", "46",
            "50", "51", "52", "55", "56", "57", "58", "59", "61", "63", "64", "65", "67", "68",
            "69", "71", "72", "76", "77", "79", "80", "81", "82", "83", "84", "86", "87", "89",
            "90", "91", "92", "93", "94", "97", "99",
        ];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
