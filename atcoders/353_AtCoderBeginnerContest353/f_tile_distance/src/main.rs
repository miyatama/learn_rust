use proconio::input;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufWriter, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BigTilePos {
    x: u64,
    y: u64,
}

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
        2 => 0,
        _ => get_k_many_move(k, &s, &t),
    };
    writeln!(w, "{}", result).unwrap();
}

fn get_k_many_move(k: u64, s: &Vec<u64>, t: &Vec<u64>) -> u64 {
    let start_big_tiles = get_neibor_big_tile(k, s);
    let terminate_big_tiles = get_neibor_big_tile(k, t);

    // 大タイルへの移動を考える
    let mut min_big_tile_move_count = u64::MAX;
    let mut start_big_tiles_index = 0;
    let mut terminate_big_tiles_index = 0;
    for i in 0..start_big_tiles.len() {
        for j in 0..terminate_big_tiles.len() {
            eprintln!("s[{}] -> t[{}], ({:?} => {:?}", i, j, &start_big_tiles[i], &terminate_big_tiles[j]);
            match calc_big_tile_arrival_count(&start_big_tiles[i], &terminate_big_tiles[j]) {
                Some(count) => {
                    if count < min_big_tile_move_count {
                        min_big_tile_move_count = count;
                        start_big_tiles_index = i;
                        terminate_big_tiles_index = j;
                    }
                }
                None => {}
            }
        }
    }

    let s_count =
        get_count_pos_to_neibor_big_tile(k, s[0], s[1], &start_big_tiles[start_big_tiles_index]);
    let t_count = get_count_pos_to_neibor_big_tile(
        k,
        t[0],
        t[1],
        &terminate_big_tiles[terminate_big_tiles_index],
    );
    min_big_tile_move_count * 2 + s_count + t_count
}

/**
 * 斜め移動の到達手数を求める
 */
fn calc_big_tile_arrival_count(s: &BigTilePos, t: &BigTilePos) -> Option<u64> {
    let moves: Vec<(i64, i64)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
    // 位置と手数を保持する
    let mut queue: VecDeque<(BigTilePos, u64)> = VecDeque::new();
    queue.push_front((s.clone(), 0));
    let mut hs: HashSet<BigTilePos> = HashSet::new();
    hs.insert(s.clone());

    // 始点と終点の関係を保持する
    let x_direction = s.x <= t.x;
    let y_direction = s.y <= t.y;
    let limit_distance = calc_big_tile_distaince(&s, &t);

    let mut max_loop = 10;

    while let Some((pos, count)) = queue.pop_front() {
        max_loop -= 1;
        if max_loop <= 0 {
            break;
        }
        if pos == *t {
            return Some(count);
        }

        for i in 0..moves.len() {
            match get_big_tile_pos_after_move(&pos, moves[i].0, moves[i].1) {
                Some(new_pos) => {
                    if x_direction == (s.x <= new_pos.x)
                        && y_direction == (s.y <= new_pos.y)
                        && !hs.contains(&new_pos)
                        && limit_distance >= calc_big_tile_distaince(&s, &new_pos)
                    {
                        eprintln!("bfs - new position: {:?}", new_pos);
                        queue.push_front((new_pos.clone(), count + 1));
                    }
                }
                None => {}
            }
        }
    }
    None
}

/**
 * 隣接する大タイルの取得
 * 条件: k >= 2
 */
fn get_neibor_big_tile(k: u64, s: &Vec<u64>) -> Vec<BigTilePos> {
    let big_tile_pos = get_big_tile_pos(k, s[0], s[1]);
    if is_big_tile(k, s[0], s[1]) {
        return vec![big_tile_pos];
    }

    // 小タイルの場合は隣接する大タイルを返す
    vec![
        get_big_tile_pos_after_move(&big_tile_pos, -1, 0),
        get_big_tile_pos_after_move(&big_tile_pos, 1, 0),
        get_big_tile_pos_after_move(&big_tile_pos, 0, -1),
        get_big_tile_pos_after_move(&big_tile_pos, 0, 1),
    ]
    .iter()
    .filter(|value| value.is_some())
    .map(|value| value.unwrap())
    .collect::<Vec<BigTilePos>>()
}

fn get_big_tile_pos_after_move(pos: &BigTilePos, delta_x: i64, delta_y: i64) -> Option<BigTilePos> {
    if pos.x <= 0 && delta_x < 0 {
        return None;
    }
    if pos.y <= 0 && delta_y < 0 {
        return None;
    }
    Some(BigTilePos {
        x: (pos.x as i64 + delta_x) as u64,
        y: (pos.y as i64 + delta_y) as u64,
    })
}

fn get_count_pos_to_neibor_big_tile(k: u64, x: u64, y: u64, to_big_tile_pos: &BigTilePos) -> u64 {
    eprintln!(
        "get_count_pos_to_neibor_big_tile: k: {}, x: {}, y: {}",
        k, x, y
    );
    if is_big_tile(k, x, y) {
        eprintln!("get_count_pos_to_neibor_big_tile: is big tile");
        return 0;
    }
    let big_tile_pos = get_big_tile_pos(k, x, y);
    eprintln!("big tile pos: {:?}", &big_tile_pos);
    // 右へ移動
    if big_tile_pos.x < to_big_tile_pos.x {
        return (big_tile_pos.x + 1) * k - x;
    }

    // 左へ移動
    if big_tile_pos.x > to_big_tile_pos.x {
        return x - big_tile_pos.x * k + 1;
    }

    // 上へ移動
    if big_tile_pos.y < to_big_tile_pos.y {
        return (big_tile_pos.y + 1) * k - y;
    }

    // 下へ移動
    y - big_tile_pos.y * k + 1
}

fn get_big_tile_pos(k: u64, x: u64, y: u64) -> BigTilePos {
    BigTilePos { x: x / k, y: y / k }
}

fn is_big_tile(k: u64, x: u64, y: u64) -> bool {
    (x / k + y / k) % 2 != 0
}

fn get_delta(factor1: u64, factor2: u64) -> u64 {
    if factor1 > factor2 {
        factor1 - factor2
    } else {
        factor2 - factor1
    }
}

fn calc_big_tile_distaince(a: &BigTilePos, b: &BigTilePos) -> u64 {
    let x_delta = get_delta(a.x, b.x);
    let y_delta = get_delta(a.y, b.y);
    calc_squrt(x_delta * x_delta + y_delta * y_delta).unwrap()
}

fn calc_squrt(n: u64) -> Option<u64> {
    let mut factor = 0u64;
    while factor.pow(2) < n {
        factor += 1;
        let val = factor.pow(2);
        if val >= n {
            return Some(factor);
        }
        if val > n {
            break;
        }
    }
    return Some(factor);
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

    #[test]
    fn test_get_count_pos_to_neibor_big_tile_01() {
        let k = 3;
        let x = 3;
        let y = 0;
        let big_tile_pos = BigTilePos { x: 0, y: 0 };
        let actual = get_count_pos_to_neibor_big_tile(k, x, y, &big_tile_pos);
        let expect = 0;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_count_pos_to_neibor_big_tile_02() {
        let k = 3;
        let x = 4;
        let y = 4;
        let big_tile_pos = BigTilePos { x: 0, y: 1 };
        let actual = get_count_pos_to_neibor_big_tile(k, x, y, &big_tile_pos);
        let expect = 2;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_count_pos_to_neibor_big_tile_03() {
        let k = 3;
        let x = 4;
        let y = 4;
        let big_tile_pos = BigTilePos { x: 2, y: 1 };
        let actual = get_count_pos_to_neibor_big_tile(k, x, y, &big_tile_pos);
        let expect = 2;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_count_pos_to_neibor_big_tile_04() {
        let k = 3;
        let x = 4;
        let y = 4;
        let big_tile_pos = BigTilePos { x: 1, y: 0 };
        let actual = get_count_pos_to_neibor_big_tile(k, x, y, &big_tile_pos);
        let expect = 2;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_get_count_pos_to_neibor_big_tile_05() {
        let k = 3;
        let x = 4;
        let y = 4;
        let big_tile_pos = BigTilePos { x: 1, y: 2 };
        let actual = get_count_pos_to_neibor_big_tile(k, x, y, &big_tile_pos);
        let expect = 2;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_big_tile_arrival_count_slash_01() {
        // 斜めのみの移動(右上)
        let start_pos = BigTilePos{x: 0, y: 0};
        let terminate_pos = BigTilePos{x: 3, y: 3};
        let actual = calc_big_tile_arrival_count(&start_pos, &terminate_pos);
        let expect = 3;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_big_tile_arrival_count_slash_02() {
        // 斜めのみの移動(右下)
        let start_pos = BigTilePos{x: 0, y: 4};
        let terminate_pos = BigTilePos{x: 4, y: 0};
        let actual = calc_big_tile_arrival_count(&start_pos, &terminate_pos);
        let expect = 4;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_big_tile_arrival_count_slash_03() {
        // 斜めのみの移動(左上)
        let start_pos = BigTilePos{x: 4, y: 0};
        let terminate_pos = BigTilePos{x: 0, y: 4};
        let actual = calc_big_tile_arrival_count(&start_pos, &terminate_pos);
        let expect = 4;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_big_tile_arrival_count_slash_04() {
        // 斜めのみの移動(左下)
        let start_pos = BigTilePos{x: 4, y: 0};
        let terminate_pos = BigTilePos{x: 0, y: 4};
        let actual = calc_big_tile_arrival_count(&start_pos, &terminate_pos);
        let expect = 4;
        assert_eq!(expect, actual);
    }


    #[test]
    fn test_calc_big_tile_arrival_count_horizontal_01() {
        // 横の移動(右)
        let start_pos = BigTilePos{x: 0, y: 0};
        let terminate_pos = BigTilePos{x: 4, y: 0};
        let actual = calc_big_tile_arrival_count(&start_pos, &terminate_pos);
        let expect = 4;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_big_tile_arrival_count_horizontal_02() {
        // 横の移動(左)
        let start_pos = BigTilePos{x: 4, y: 4};
        let terminate_pos = BigTilePos{x: 0, y: 4};
        let actual = calc_big_tile_arrival_count(&start_pos, &terminate_pos);
        let expect = 4;
        assert_eq!(expect, actual);
    }
}
