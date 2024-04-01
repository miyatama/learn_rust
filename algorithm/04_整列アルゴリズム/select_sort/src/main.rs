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
    let list = select_sort(a);
    list.iter().for_each(|val| {
        writeln!(w, "{}", val).unwrap();
    });
}

fn select_sort(src_list: Vec<u64>) -> Vec<u64> {
    let get_max_position = |list: Vec<u64>| -> usize {
        let max_value = list.iter().max().unwrap();
        list.iter().position(|val| val == max_value).unwrap()
    };
    let swap_value = |list: &mut Vec<u64>, p1: usize, p2: usize| -> Vec<u64> {
        let value = list[p1];
        list[p1] = list[p2];
        list[p2] = value;
        list.clone()
    };
    let mut list: Vec<u64> = src_list.iter().copied().collect();
    let list_len = src_list.len();
    for i in 1..=list_len {
        let limit_len = (list_len - i) as usize;
        let limit_list = list
            .iter()
            .enumerate()
            .filter(|(i, _)| *i <= limit_len)
            .map(|(_, val)| *val)
            .collect::<Vec<u64>>();
        let position = get_max_position(limit_list);
        list = swap_value(&mut list, src_list.len() - i, position);
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
        let expect = vec!["1", "2", "3", "4", "5", "6", "7", "8"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
