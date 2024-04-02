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
    let list = heap_sort(a);
    eprintln!("result: {:?}", list);
    list.iter().for_each(|val| {
        writeln!(w, "{}", val).unwrap();
    });
}

fn heap_sort(src_list: Vec<u64>) -> Vec<u64> {
    let list_size = src_list.len();
    let mut list = build_heap(src_list, list_size);
    eprintln!("builded: {:?}", &list);
    for i in (1..list_size).rev() {
        list = swap(&mut list, 0, i);
        list = heapify(list, 0, i);
    }
    list
}

fn swap(list: &mut Vec<u64>, p1: usize, p2: usize) -> Vec<u64> {
    let value = list[p1];
    list[p1] = list[p2];
    list[p2] = value;
    list.clone()
}

fn build_heap(list: Vec<u64>, list_size: usize) -> Vec<u64> {
    let mut list = list.clone();
    for i in (0..=(list_size / 2 - 1)).rev() {
        list = heapify(list, i, list_size);
    }
    list
}

fn heapify(list: Vec<u64>, i: usize, max: usize) -> Vec<u64> {
    let mut list = list;
    let mut largest = i;
    let left = i * 2 + 1;
    let right = i * 2 + 2;
    if left < max && list[left] > list[i] {
        largest = left;
    }
    if right < max && list[right] > list[largest] {
        largest = right;
    }

    if largest != i {
        list = swap(&mut list, i, largest);
        list = heapify(list, largest, max);
    }
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
            10,
            vec![
                4, 77, 88, 1, 90, 93, 49, 4, 41, 77, 75, 54, 54, 73, 30, 36, 27, 87, 70, 97, 99,
                72, 58, 80, 24, 65, 8, 57, 90, 37, 55, 63, 60, 99, 74, 34, 41, 10, 8, 24, 18, 21,
                45, 46, 85, 65, 79, 86, 87, 54, 21, 11, 29, 74, 78, 99, 56, 46, 6, 98, 9, 72, 57,
                28, 51, 47, 78, 41, 40, 81, 88, 59, 41, 41, 54, 0, 22, 17, 29, 49, 75, 36, 36, 1,
                53, 5, 98, 51, 26, 62, 97, 14, 43, 57, 1, 76, 32, 49, 48, 23,
            ],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec![
            "0", "1", "1", "1", "4", "4", "5", "6", "8", "8", "9", "10", "11", "14", "17", "18",
            "21", "21", "22", "23", "24", "24", "26", "27", "28", "29", "29", "30", "32", "34",
            "36", "36", "36", "37", "40", "41", "41", "41", "41", "41", "43", "45", "46", "46",
            "47", "48", "49", "49", "49", "51", "51", "53", "54", "54", "54", "54", "55", "56",
            "57", "57", "57", "58", "59", "60", "62", "63", "65", "65", "70", "72", "72", "73",
            "74", "74", "75", "75", "76", "77", "77", "78", "78", "79", "80", "81", "85", "86",
            "87", "87", "88", "88", "90", "90", "93", "97", "97", "98", "98", "99", "99", "99",
        ];

        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
