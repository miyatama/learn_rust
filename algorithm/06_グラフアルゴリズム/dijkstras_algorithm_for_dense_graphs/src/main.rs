use self::logics::dijkstras_fdg;
use std::io::{self, BufWriter, Write};

pub mod logics;
fn main() {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let n = 6;
    let s = 0;
    let t = 5;
    let nodes = vec![
        (0, vec![(1, 6), (2, 8), (3, 18)]),
        (1, vec![(4, 11)]),
        (2, vec![(3, 9)]),
        (3, vec![]),
        (4, vec![(5, 3)]),
        (5, vec![(3, 4), (2, 7)]),
    ];
    dijkstras_fdg::search(&mut stdout, n, s, t, nodes);
    stdout.flush().unwrap();
}
