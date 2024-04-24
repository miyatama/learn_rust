#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellValue {
    // 未入力
    None,

    // User
    Player,

    // AI
    Enemy,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameBoardState {
    Win,
    Lose,
    Draw,
}

pub fn best_move(board: &Vec<CellValue>) -> Option<usize> {
    let ply = 10u8;
    let (best_move, _) = alpha_beta(board, CellValue::Enemy, ply, i64::MIN + 1, i64::MAX - 1);
    best_move
}

pub fn is_player_win(board: &Vec<CellValue>, player: CellValue) -> bool {
    contains_player_line(board, player) == GameBoardState::Win
}

pub fn print_board(board: &Vec<CellValue>) {
    let board = board
        .iter()
        .map(|value| match value {
            CellValue::None => " ",
            CellValue::Player => "o",
            CellValue::Enemy => "x",
        })
        .collect::<Vec<_>>();
    println!("-------");
    println!("|{}|{}|{}|", board[0], board[1], board[2]);
    println!("-- - --");
    println!("|{}|{}|{}|", board[3], board[4], board[5]);
    println!("-- - --");
    println!("|{}|{}|{}|", board[6], board[7], board[8]);
    println!("-------");
}

pub fn none_cell_not_exists(board: &Vec<CellValue>) -> bool {
    board
        .iter()
        .filter(|cell| **cell == CellValue::None)
        .next()
        .is_none()
}

fn alpha_beta(
    board: &Vec<CellValue>,
    player: CellValue,
    ply: u8,
    low: i64,
    high: i64,
) -> (Option<usize>, Option<i64>) {
    if ply <= 0 || none_cell_not_exists(board) {
        return (None, Some(evaluate_score(board, player)));
    }

    let mut low = low;
    let mut best: (Option<usize>, Option<i64>) = (None, None);
    let indexes = board
        .iter()
        .enumerate()
        .filter(|(_, cell)| **cell == CellValue::None)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();
    for i in 0..indexes.len() {
        let mut new_board = board.clone();
        new_board[indexes[i]] = player;
        let (_, score) = alpha_beta(&new_board, get_opponent(player), ply - 1, -high, -low);
        if score.is_none() {
            continue;
        }
        let score = score.unwrap();
        if best.1.is_none() || -score > best.1.unwrap() {
            low = -score;
            best = (Some(indexes[i]), Some(-low));
        }
        if low >= high {
            return best;
        }
    }
    best
}

fn get_opponent(player: CellValue) -> CellValue {
    if player == CellValue::Player {
        CellValue::Enemy
    } else {
        CellValue::Player
    }
}

fn none_cell_len(board: &Vec<CellValue>) -> i64 {
    board
        .iter()
        .filter(|cell| **cell == CellValue::None)
        .collect::<Vec<_>>()
        .len() as i64
}

fn evaluate_score(board: &Vec<CellValue>, player: CellValue) -> i64 {
    match is_win_game(board, player) {
        GameBoardState::Win => 100 + none_cell_len(board),
        GameBoardState::Lose => -100 - none_cell_len(board),
        GameBoardState::Draw => calculate_draw_score(board, player),
    }
}

fn is_win_game(board: &Vec<CellValue>, player: CellValue) -> GameBoardState {
    let ai_win = contains_player_line(board, player);
    let player_win = contains_player_line(board, get_opponent(player));
    match (ai_win, player_win) {
        (GameBoardState::Win, GameBoardState::Win) => GameBoardState::Draw,
        (GameBoardState::Win, _) => GameBoardState::Win,
        (_, GameBoardState::Win) => GameBoardState::Lose,
        _ => GameBoardState::Draw,
    }
}

fn contains_player_line(board: &Vec<CellValue>, player: CellValue) -> GameBoardState {
    let lines = get_valid_lines();
    for i in 0..lines.len() {
        if board[lines[i][0]] == player
            && board[lines[i][1]] == player
            && board[lines[i][2]] == player
        {
            return GameBoardState::Win;
        }
    }
    GameBoardState::Draw
}

fn get_valid_lines() -> Vec<Vec<usize>> {
    vec![
        // よっこ
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        // たって
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        // なっなめ
        vec![0, 4, 8],
        vec![2, 4, 6],
    ]
}

fn assignable_player_line_count(board: &Vec<CellValue>, player: CellValue) -> i64 {
    let mut count = 0i64;
    let opponent = get_opponent(player);
    let lines = get_valid_lines();
    for i in 0..lines.len() {
        if board[lines[i][0]] != opponent
            && board[lines[i][1]] != opponent
            && board[lines[i][2]] != opponent
        {
            count += 1;
        }
    }
    count
}

fn calculate_draw_score(board: &Vec<CellValue>, player: CellValue) -> i64 {
    assignable_player_line_count(board, player)
        - assignable_player_line_count(board, get_opponent(player))
}
