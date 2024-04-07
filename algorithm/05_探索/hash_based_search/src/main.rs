use self::logics::{hash_based_search_string, hash_based_search_u64};
use proconio::input;
use std::io::{self, BufWriter, Write};

pub mod logics;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
        v: u64,
        sn: u64,
        sa: [String; n],
        sv: String
    }

    // call u64 search
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    hash_based_search_u64::search(&mut stdout, n, a, v);
    stdout.flush().unwrap();

    // call string search
    let string_stdout = io::stdout();
    let mut string_stdout = BufWriter::new(string_stdout.lock());
    hash_based_search_string::search(&mut string_stdout, sn, sa, sv);
    string_stdout.flush().unwrap();
}
