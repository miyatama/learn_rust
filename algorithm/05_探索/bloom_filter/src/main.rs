use self::logics::bloom_filter;
use proconio::input;
use std::io::{self, BufWriter, Write};

pub mod logics;

fn main() {
    input! {
        n: u64,
        a: [String; n],
        v: String
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    bloom_filter::search(&mut stdout, n, a, v);
    stdout.flush().unwrap();
}
