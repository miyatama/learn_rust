use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        k: u64,
        s: [u64; 2],
        t: [u64; 2],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, k, s, t);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, k: u64, s: Vec<u64>, t: Vec<u64>) {
    let result = match k {
        1 => get_delta(s[0], t[0]) + get_delta(s[1], t[1]),
        2 => get_k2_move(k, s, t),
        _ => get_k_many_move(k, s, t),
    };
    writeln!(w, "{}", result).unwrap();
}

fn get_k2_move(k: u64, s: Vec<u64>, t: Vec<u64>) -> u64 {
    let s_big_tile = get_big_tile_pos(k, s[0], s[1]);
    let t_big_tile = get_big_tile_pos(k, t[0], t[1]);
    let big_tile_delta = [
        get_delta(s_big_tile.0, t_big_tile.0),
        get_delta(s_big_tile.1, t_big_tile.1),
    ];
    let is_s_big_tile = is_big_tile(s[0], s[1]);
    let is_t_big_tile = is_big_tile(t[0], t[1]);
    let mut route_adding = 0;
    for i in 0..2 {
        // sとtが隣接
        if big_tile_delta[i] <= 1 {
            continue;
        }

        // 1マス以上離れている
        let same_s_tile = (big_tile_delta[i] - 1) / 2;
        route_adding += if is_s_big_tile {
            big_tile_delta[i] - 1 - same_s_tile
        } else {
            same_s_tile
        };
    }
    route_adding += if !is_s_big_tile {
        // x方向(右へ出るか左へ出るか)
        // y方向(上へ出るか下へ出るか)
    } else {
        0
    };
    route_adding += if !is_t_big_tile {
        // x方向(右から入るか左から入ってくるか)
        // y方向(上から入るか下から入ってくるか)
    } else {
        0
    };
    route_adding
}

fn get_k_many_move(k: u64, s: Vec<u64>, t: Vec<u64>) -> u64 {
    0
}

fn get_big_tile_pos(k: u64, x: u64, y: u64) -> (u64, u64) {
    (x / k, y / k)
}

fn is_big_tile(k: u64, x: u64, y: u64) -> bool {
    (x / k + y / k) % 2 != 0
}

fn is_big_tile_big_pos(x: u64, y: u64) -> bool {
    (x + y) % 2 != 0
}

fn get_delta(factor1: u64, factor2: u64) -> u64 {
    if factor1 > factor2 {
        factor1 - factor2
    } else {
        factor2 - factor1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 3, vec![7, 2], vec![1, 6]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["5"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 1, vec![41, 42], vec![13, 56]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["42"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 100, vec![100, 99], vec![199, 1]);

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            96929423,
            vec![5105216413055191, 10822465733465225],
            vec![1543712011036057, 14412421458305526],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["79154049"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_is_big_tile01() {
        let actual = is_big_tile(3, 0, 0);
        assert_eq!(false, actual);
    }

    #[test]
    fn test_is_big_tile02() {
        let actual = is_big_tile(3, 3, 0);
        assert_eq!(true, actual);
    }

    #[test]
    fn test_is_big_tile03() {
        let actual = is_big_tile(3, 0, 3);
        assert_eq!(true, actual);
    }
}
