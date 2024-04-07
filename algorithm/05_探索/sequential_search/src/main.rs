use self::logics::sequential_search;
use proconio::input;
use std::io::{self, BufWriter, Write};

pub mod logics;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
        v: u64
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    sequential_search::search(&mut stdout, n, a, v);
    stdout.flush().unwrap();
}
