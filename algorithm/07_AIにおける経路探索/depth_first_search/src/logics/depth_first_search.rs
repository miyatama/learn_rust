use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

pub fn think(board: &Vec<u8>) -> Option<Vec<Direction>> {
    search(board, u32::MAX - 1)
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

fn search(initial: &Vec<u8>, max_depth: u32) -> Option<Vec<Direction>> {
    if is_goarl(initial) {
        return Some(vec![]);
    }
    let mut stack: VecDeque<(Vec<u8>, u32)> = VecDeque::new();
    let mut set: HashSet<Vec<u8>> = HashSet::new();
    let mut from_state: HashMap<Vec<u8>, (Vec<u8>, Direction)> = HashMap::new();
    let mut solution_directions: Vec<Direction> = Vec::new();
    stack.push_back((initial.to_vec(), 0));
    while let Some(state) = stack.pop_back() {
        let (base_board, depth) = state;
        set.insert(base_board.clone());
        let directions = get_valid_direction(&base_board);
        for i in 0..directions.len() {
            let new_board = apply_direction(&base_board, directions[i]);
            if set.contains(&new_board) {
                continue;
            }
            if is_goarl(&new_board) {
                from_state.insert(new_board.clone(), (base_board.clone(), directions[i]));
                stack.clear();
                break;
            }
            if depth < max_depth {
                from_state.insert(new_board.clone(), (base_board.clone(), directions[i]));
                stack.push_back((new_board.clone(), depth + 1));
            }
        }
    }
    let mut from_board = get_goal();
    if from_state.contains_key(&from_board) {
        loop {
            let (board, direction) = from_state.get(&from_board).unwrap();
            solution_directions.push(*direction);
            from_board = board.to_vec();
            if equal_state(initial, board) {
                break;
            }
        }
        solution_directions = solution_directions.into_iter().rev().collect::<Vec<Direction>>();
        return Some(solution_directions);
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

fn equal_state(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    a.iter()
        .zip(b.iter())
        .filter(|(&a_val, &b_val)| a_val != b_val)
        .next()
        .is_none()
}

fn is_goarl(board: &Vec<u8>) -> bool {
    let goal = get_goal();
    equal_state(board, &goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_goal_success() {
        let board = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
        let actual = is_goarl(&board);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_goal_failure() {
        let board = vec![8, 1, 3, 2, 4, 5, 0, 7, 6];
        let actual = is_goarl(&board);
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
}
