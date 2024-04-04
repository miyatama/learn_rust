use proconio::input;
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
    let list = merge_sort(a);
    eprintln!("result: {:?}", list);
    list.iter().for_each(|val| {
        writeln!(w, "{}", val).unwrap();
    });
}

fn merge_sort(src_list: Vec<u64>) -> Vec<u64> {
    let len = src_list.len();
    if len < 2 {
        return src_list;
    }
    if len == 2 {
        if src_list[0] < src_list[1] {
            return src_list;
        } else {
            return vec![src_list[1], src_list[0]];
        }
    }

    let index = len / 2;
    let l1 = merge_sort(src_list[0..index].to_vec());
    let l2 = merge_sort(src_list[index..len].to_vec());
    merge(l1, l2)
}

fn merge(l1: Vec<u64>, l2: Vec<u64>) -> Vec<u64> {
    let l1_len = l1.len();
    let l2_len = l2.len();
    if l1_len == 0 && l2_len == 0 {
        return vec![];
    }
    if l1_len == 0 {
        return l2;
    }
    if l2_len == 0 {
        return l1;
    }

    let l1_value = l1[0];
    let l2_value = l2[0];
    let mut retain_l1 = l1.clone();
    let mut retain_l2 = l2.clone();
    let mut sorted_list = vec![];
    if l1_value > l2_value {
        sorted_list.push(l2_value);
        retain_l2.remove(0);
    } else {
        sorted_list.push(l1_value);
        retain_l1.remove(0);
    }

    let merged_list = merge(retain_l1, retain_l2);
    sorted_list.extend(merged_list);
    sorted_list
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
