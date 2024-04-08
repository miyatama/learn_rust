use self::logics::binary_search_tree;
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
    binary_search_tree::search(&mut stdout, n, a, v);
    stdout.flush().unwrap();
}
