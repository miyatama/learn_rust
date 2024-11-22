use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        x: u64,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, a, b,c, x);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, a: u64, b: u64, c: u64, x: u64) {
    let mut count = 0;
    for i in 0..=a { 
        for j in 0..=b {
            for k in 0..=c {
                let value = i * 500 + j * 100 + k * 50;
                if value == x {
                    count += 1;
                }
            }
        }
    }
    writeln!(w, "{}", count).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 2, 2, 2, 100);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}