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
    let list = insertion_sort(a);
    list.iter().for_each(|val| {
        writeln!(w, "{}", val).unwrap();
    });
}

fn insertion_sort(src_list: Vec<u64>) -> Vec<u64> {
    let insert = |list: Vec<u64>, index: usize, value: u64| -> Vec<u64> {
        let mut lower: Vec<_> = list
            .iter()
            .enumerate()
            .filter(|(i, val)| *i < index && **val < value)
            .map(|(_, val)| *val)
            .collect();
        let greter: Vec<_> = list
            .iter()
            .enumerate()
            .filter(|(i, val)| *i < index && **val >= value)
            .map(|(_, val)| *val)
            .collect();
        let after: Vec<_> = list
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > index)
            .map(|(_, val)| *val)
            .collect();
        let current = vec![value];
        lower.extend(current);
        lower.extend(greter);
        lower.extend(after);
        lower
    };
    let mut list: Vec<u64> = src_list.iter().copied().collect();
    for i in 1..src_list.len() {
        list = insert(list.clone(), i, list[i]);
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
