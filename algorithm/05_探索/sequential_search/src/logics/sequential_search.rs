use std::io::{Write};

pub fn search<W: Write>(w: &mut W, _n: u64, a: Vec<u64>, value: u64) {
    let result = sequential_search(a, value);
    match result {
        None => writeln!(w, "Not Found").unwrap(),
        Some(index) => writeln!(w, "{}", index).unwrap(),
    }
}

fn sequential_search(src_list: Vec<u64>, value: u64) -> Option<usize> {
    let mut result = None;
    for i in 0..src_list.len() {
        if src_list[i] == value {
            result = Some(i);
            break;
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
        search(&mut buff, 10, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 1);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["9"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search02() {
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
            23,
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["99"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search03() {
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