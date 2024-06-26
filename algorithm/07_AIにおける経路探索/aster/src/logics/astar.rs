use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
pub enum Evaluation {
    FairEvaluator,
    GoodEvaluator,
    WeakEvaluator,
    BadEvaluator,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u64,
    depth: u64,
    board: Vec<u8>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // cost -> depthの順で判定
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.depth.cmp(&other.depth))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn print_board(board: &Vec<u8>) {
    let board = board
        .iter()
        .map(|&val| {
            if val == 0 {
                " ".to_string()
            } else {
                val.to_string()
            }
        })
        .collect::<Vec<String>>();
    println!("-------");
    println!("|{}|{}|{}|", board[0], board[1], board[2]);
    println!("|{}|{}|{}|", board[3], board[4], board[5]);
    println!("|{}|{}|{}|", board[6], board[7], board[8]);
    println!("-------");
}

pub fn think(board: &Vec<u8>, evaluation: &Evaluation) -> Option<Vec<Direction>> {
    search(board, evaluation)
}

pub fn apply_direction(board: &Vec<u8>, direction: Direction) -> Vec<u8> {
    let space_cell_position = board.iter().position(|&val| val == 0).unwrap();
    match direction {
        Direction::Up => swap_space(board, space_cell_position, space_cell_position - 3),
        Direction::Down => swap_space(board, space_cell_position, space_cell_position + 3),
        Direction::Left => swap_space(board, space_cell_position, space_cell_position - 1),
        Direction::Right => swap_space(board, space_cell_position, space_cell_position + 1),
    }
}

fn swap_space(board: &Vec<u8>, from: usize, to: usize) -> Vec<u8> {
    let mut board = board.to_vec();
    board.swap(from, to);
    board.to_vec()
}

fn search(initial: &Vec<u8>, evaluation: &Evaluation) -> Option<Vec<Direction>> {
    if is_goal(initial) {
        return Some(vec![]);
    }
    let mut opens: BinaryHeap<State> = BinaryHeap::new();
    let mut closes: HashSet<Vec<u8>> = HashSet::new();
    let mut from_state: HashMap<Vec<u8>, (Vec<u8>, Direction)> = HashMap::new();
    opens.push(State {
        cost: 0,
        depth: 0,
        board: initial.to_vec(),
    });
    while let Some(state) = opens.pop() {
        closes.insert(state.board.clone());
        if is_goal(&state.board) {
            break;
        }
        let directions = get_valid_direction(&state.board);
        for i in 0..directions.len() {
            let new_board = apply_direction(&state.board, directions[i]);
            if closes.contains(&new_board) {
                continue;
            }
            let same_state = get_state(&opens, &new_board);
            let exists_same_state = same_state.is_some();
            let new_board_cost = calculate_cost(&new_board, evaluation);
            if !exists_same_state || new_board_cost < same_state.unwrap().cost {
                if exists_same_state {
                    opens = BinaryHeap::from(
                        opens
                            .into_vec()
                            .into_iter()
                            .filter(|state| !equal_state(&state.board, &new_board))
                            .collect::<Vec<State>>(),
                    );
                }
                from_state.insert(new_board.clone(), (state.board.clone(), directions[i]));
                opens.push(State {
                    depth: state.depth + 1,
                    cost: new_board_cost,
                    board: new_board.clone(),
                });
            }
        }
    }
    let mut from_board = get_goal();
    if from_state.contains_key(&from_board) {
        let mut solution_directions: Vec<Direction> = Vec::new();
        loop {
            let (board, direction) = from_state.get(&from_board).unwrap();
            solution_directions.push(*direction);
            from_board = board.to_vec();
            if equal_state(initial, board) {
                break;
            }
        }
        solution_directions = solution_directions
            .into_iter()
            .rev()
            .collect::<Vec<Direction>>();
        return Some(solution_directions);
    }
    None
}

fn calculate_cost(board: &Vec<u8>, evaluation: &Evaluation) -> u64 {
    match *evaluation {
        Evaluation::FairEvaluator => calculate_fair_evaluator_cost(board),
        Evaluation::GoodEvaluator => calculate_good_evaluator_cost(board),
        Evaluation::WeakEvaluator => calculate_weak_evaluator_cost(board),
        Evaluation::BadEvaluator => calculate_bad_evaluator_cost(board),
    }
}

fn calculate_fair_evaluator_cost(board: &Vec<u8>) -> u64 {
    let goal_positions = get_goal_cell_positions();
    let mut board_positions = board
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, value)| (value, (index % 3, index / 3)))
        .collect::<Vec<_>>();
    board_positions.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());
    goal_positions
        .iter()
        .zip(board_positions.iter())
        .map(|(a, b)| {
            let x_delta = calc_delta_usize(a.1 .0, b.1 .0);
            let y_delta = calc_delta_usize(a.1 .1, b.1 .1);
            calc_squrt(x_delta * x_delta + y_delta * y_delta)
        })
        .sum()
}

fn calculate_good_evaluator_cost(board: &Vec<u8>) -> u64 {
    let p = calculate_fair_evaluator_cost(board);
    let goal_board = get_goal();
    let check_indexes = vec![1, 2, 5, 8, 7, 6, 3];
    let s: u64 = check_indexes
        .iter()
        .map(|i| {
            let cell_value = board[*i];
            let collect_value = goal_board[*i];
            let mut delta = 0;
            if cell_value != collect_value {
                delta = 2;
            }
            if cell_value == 0 {
                delta = 1;
            }
            delta as u64
        })
        .sum();
    p + s * 3
}

fn calculate_weak_evaluator_cost(board: &Vec<u8>) -> u64 {
    let goal_board = get_goal();
    board
        .iter()
        .zip(goal_board.iter())
        .filter(|(&a, &b)| a != b)
        .collect::<Vec<_>>()
        .len() as u64
}

fn calculate_bad_evaluator_cost(board: &Vec<u8>) -> u64 {
    let sum_delta: u64 = calc_delta(board[0], board[8])
        + calc_delta(board[2], board[6])
        + calc_delta(board[1], board[7])
        + calc_delta(board[3], board[5]);

    calc_delta_u64(16u64, sum_delta)
}

fn calc_delta(a: u8, b: u8) -> u64 {
    if a > b {
        (a - b) as u64
    } else {
        (b - a) as u64
    }
}

fn calc_delta_u64(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn calc_delta_usize(a: usize, b: usize) -> u64 {
    if a > b {
        (a - b) as u64
    } else {
        (b - a) as u64
    }
}

fn calc_squrt(n: u64) -> u64 {
    let mut factor = 0u64;
    while factor.pow(2) < n {
        factor += 1;
        let val = factor.pow(2);
        if val == n {
            return factor;
        }
        if val > n {
            break;
        }
    }
    if factor > 0 {
        factor - 1
    } else {
        factor
    }
}

fn get_state(bh: &BinaryHeap<State>, board: &Vec<u8>) -> Option<State> {
    for state in bh {
        if equal_state(&state.board, board) {
            return Some(state.clone());
        }
    }
    None
}

fn get_valid_direction(board: &Vec<u8>) -> Vec<Direction> {
    let pos = board.iter().position(|&val| val == 0).unwrap();
    let mut directions: Vec<Direction> = Vec::new();
    if pos <= 5 {
        directions.push(Direction::Down);
    }
    if pos >= 3 && pos <= 8 {
        directions.push(Direction::Up);
    }
    if pos != 0 && pos != 3 && pos != 6 {
        directions.push(Direction::Left);
    }
    if pos != 2 && pos != 5 && pos != 8 {
        directions.push(Direction::Right);
    }
    directions
}

fn get_goal() -> Vec<u8> {
    vec![1, 2, 3, 8, 0, 4, 7, 6, 5]
}

fn get_goal_cell_positions() -> Vec<(u8, (usize, usize))> {
    vec![
        (0, (1, 1)),
        (1, (0, 0)),
        (2, (1, 0)),
        (3, (2, 0)),
        (4, (2, 1)),
        (5, (2, 2)),
        (6, (1, 2)),
        (7, (0, 2)),
        (8, (0, 1)),
    ]
}

fn equal_state(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    a.iter()
        .zip(b.iter())
        .filter(|(&a_val, &b_val)| a_val != b_val)
        .next()
        .is_none()
}

fn is_goal(board: &Vec<u8>) -> bool {
    let goal = get_goal();
    equal_state(board, &goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_goal_success() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = is_goal(&board);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_goal_failure() {
        let board = vec![8, 1, 3, 2, 4, 5, 0, 7, 6];
        let actual = is_goal(&board);
        assert_eq!(actual, false);
    }

    #[test]
    fn get_valid_direction_all() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = get_valid_direction(&board);
        let expect = vec![
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_left_top() {
        let board = vec![0, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Down, Direction::Right];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_right_top() {
        let board = vec![1, 2, 0, 8, 3, 4, 7, 6, 5];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Down, Direction::Left];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_left_bottom() {
        let board = vec![1, 2, 3, 8, 7, 4, 0, 6, 5];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Up, Direction::Right];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_right_bottom() {
        let board = vec![1, 2, 3, 8, 5, 4, 7, 6, 0];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Up, Direction::Left];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_left_center() {
        let board = vec![1, 2, 3, 0, 7, 4, 8, 6, 5];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Down, Direction::Up, Direction::Right];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_right_center() {
        let board = vec![1, 2, 3, 8, 5, 0, 7, 6, 4];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Down, Direction::Up, Direction::Left];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_center_top() {
        let board = vec![1, 0, 3, 2, 7, 4, 8, 6, 5];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Down, Direction::Left, Direction::Right];
        assert_eq!(actual, expect);
    }

    #[test]
    fn get_valid_direction_center_bottom() {
        let board = vec![1, 2, 3, 8, 5, 6, 7, 0, 4];
        let actual = get_valid_direction(&board);
        let expect = vec![Direction::Up, Direction::Left, Direction::Right];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_left_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let direction = Direction::Left;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 2, 3, 0, 8, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_left_02() {
        let board = vec![1, 2, 3, 8, 4, 0, 7, 6, 5];
        let direction = Direction::Left;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_right_01() {
        let board = vec![0, 2, 3, 8, 1, 4, 7, 6, 5];
        let direction = Direction::Right;
        let actual = apply_direction(&board, direction);
        let expect = vec![2, 0, 3, 8, 1, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_right_02() {
        let board = vec![1, 0, 3, 8, 2, 4, 7, 6, 5];
        let direction = Direction::Right;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 3, 0, 8, 2, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_up_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let direction = Direction::Up;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 0, 3, 8, 2, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_up_02() {
        let board = vec![1, 2, 3, 8, 6, 4, 7, 0, 5];
        let direction = Direction::Up;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_down_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let direction = Direction::Down;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 2, 3, 8, 6, 4, 7, 0, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn apply_direction_down_02() {
        let board = vec![1, 0, 3, 8, 2, 4, 7, 6, 5];
        let direction = Direction::Down;
        let actual = apply_direction(&board, direction);
        let expect = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_fair_evaluator_cost_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = calculate_fair_evaluator_cost(&board);
        let expect = 0;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_fair_evaluator_cost_02() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 5, 6];
        let actual = calculate_fair_evaluator_cost(&board);
        let expect = 2;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_fair_evaluator_cost_03() {
        let board = vec![2, 3, 8, 0, 4, 7, 6, 5, 1];
        let actual = calculate_fair_evaluator_cost(&board);
        let expect = 12;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_good_evaluator_cost_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = calculate_good_evaluator_cost(&board);
        let expect = 0;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_good_evaluator_cost_02() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 5, 6];
        let actual = calculate_good_evaluator_cost(&board);
        // P(n) = 2, S(n) = 4より、2 + 3 * 4 = 14
        let expect = 14;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_good_evaluator_cost_03() {
        let board = vec![2, 3, 8, 0, 4, 7, 6, 5, 1];
        let actual = calculate_good_evaluator_cost(&board);
        // P(n) = 12, S(n) = 13より、12 + 3 * 13 = 54
        let expect = 51;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_good_evaluator_cost_04() {
        let board = vec![1, 2, 0, 8, 3, 4, 7, 6, 5];
        let actual = calculate_good_evaluator_cost(&board);
        // P(n) = 2, S(n) = 1より、2 + 3 * 1 = 5
        let expect = 5;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_weak_evaluator_cost_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = calculate_weak_evaluator_cost(&board);
        let expect = 0;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_weak_evaluator_cost_02() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 5, 6];
        let actual = calculate_weak_evaluator_cost(&board);
        let expect = 2;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_weak_evaluator_cost_03() {
        let board = vec![2, 3, 8, 0, 4, 7, 6, 5, 1];
        let actual = calculate_weak_evaluator_cost(&board);
        let expect = 9;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_bad_evaluator_cost_01() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = calculate_bad_evaluator_cost(&board);
        let expect = 0;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_bad_evaluator_cost_02() {
        let board = vec![8, 1, 2, 7, 0, 3, 6, 5, 4];
        let actual = calculate_bad_evaluator_cost(&board);
        let expect = 0;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_bad_evaluator_cost_03() {
        let board = vec![1, 7, 5, 2, 0, 3, 4, 6, 8];
        let actual = calculate_bad_evaluator_cost(&board);
        let expect = 6;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calculate_bad_evaluator_cost_04() {
        let board = vec![8, 3, 7, 2, 5, 6, 1, 4, 0];
        let actual = calculate_bad_evaluator_cost(&board);
        let expect = 3;
        assert_eq!(actual, expect);
    }
}
