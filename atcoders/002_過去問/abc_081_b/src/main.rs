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

fn main_logic<W: Write>(w: &mut W, n: u64, a: Vec<u64>) {
    let mut a = a;
    let mut count = 0;
    while true {
        match divide(a) {
            None => {
                break;
            }
            Some(new_a) => {
                count += 1;
                a = new_a;
            }
        }
    }
    writeln!(w, "{}", count).unwrap();
}

fn divide(a: Vec<u64>) -> Option<Vec<u64>> {
    if a.iter().filter(|value| (*value % 2) != 0).count() > 0 {
        return None;
    }
    Some(a.iter().map(|value| *value / 2).collect::<Vec<u64>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, vec![16, 12, 24]);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
