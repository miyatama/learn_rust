use std::io::{Write};

pub fn search<W: Write>(w: &mut W, _n: u64, a: Vec<u64>, value: u64) {
    let result = binary_search(a, value);
    match result {
        None => writeln!(w, "Not Found").unwrap(),
        Some(index) => writeln!(w, "{}", index).unwrap(),
    }
}

fn binary_search(src_list: Vec<u64>, value: u64) -> Option<usize> {
    let mut result = None;
    let mut min_index = 0;
    let mut max_index = src_list.len() - 1;
    while min_index <= max_index {
        let index = (min_index + max_index) / 2;
        let match_value = src_list[index];
        eprintln!(
            "range: {} - {}, search: {}, match: {}[{}]",
            min_index, max_index, value, match_value, index
        );
        if match_value == value {
            result = Some(index);
            break;
        }
        match match_value {
            v if v < value => min_index = index + 1,
            _ => max_index = index - 1,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search01() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search02() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["9"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search03() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 10, vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11], 2);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Not Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search04() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 3, vec![1, 2, 3], 3);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }


    #[test]
    fn test_search05() {
        let mut buff = Vec::<u8>::new();
        search(
            &mut buff,
            10,
            vec![
                0, 1, 4, 5, 6, 8, 9, 10, 11, 14, 17, 18, 21, 22, 23, 24, 26, 27, 28, 29, 30, 32,
                34, 36, 37, 40, 41, 43, 45, 46, 47, 48, 49, 51, 53, 54, 55, 56, 57, 58, 59, 60, 62,
                63, 65, 70, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 85, 86, 87, 88, 90, 93, 97, 98,
                99,
            ],
            99,
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["64"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search06() {
        let mut buff = Vec::<u8>::new();
        search(
            &mut buff,
            10,
            vec![
                4, 77, 88, 1, 90, 93, 49, 4, 41, 77, 75, 54, 54, 73, 30, 36, 27, 87, 70, 97, 99,
                72, 58, 80, 24, 65, 8, 57, 90, 37, 55, 63, 60, 99, 74, 34, 41, 10, 8, 24, 18, 21,
                45, 46, 85, 65, 79, 86, 87, 54, 21, 11, 29, 74, 78, 99, 56, 46, 6, 98, 9, 72, 57,
                28, 51, 47, 78, 41, 40, 81, 88, 59, 41, 41, 54, 0, 22, 17, 29, 49, 75, 36, 36, 1,
                53, 5, 98, 51, 26, 62, 97, 14, 43, 57, 1, 76, 32, 49, 48, 23,
            ],
            300,
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Not Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}