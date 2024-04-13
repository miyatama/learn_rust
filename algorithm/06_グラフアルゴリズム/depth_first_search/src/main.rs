use self::logics::depth_first_search;
use proconio::input;
use std::io::{self, BufWriter, Write};

pub mod logics;

fn main() {
    /*
    input! {
        n: u64,
        s: u64,
        t: u64,
        vertexes: [(u64, [u64]); n]
    }
     */

    let n = 16;
    let s = 0;
    let t = 15;
    let vertexes = vec![
        (0, vec![1, 6, 8]),
        (1, vec![0, 2, 3]),
        (6, vec![0, 5, 7]),
        (8, vec![0, 7, 14]),
        (2, vec![1, 11, 10]),
        (3, vec![1, 4, 12]),
        (5, vec![4, 6, 9]),
        (4, vec![3, 5, 13]),
        (7, vec![6, 8, 9]),
        (9, vec![5, 7, 15]),
        (10, vec![2]),
        (11, vec![2]),
        (12, vec![3]),
        (13, vec![4]),
        (14, vec![8]),
        (15, vec![9]),
    ];
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    depth_first_search::search(&mut stdout, n, s, t, vertexes);
    stdout.flush().unwrap();
}
