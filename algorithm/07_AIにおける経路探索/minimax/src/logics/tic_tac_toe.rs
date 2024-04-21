use super::minimax::{self, CellValue};
use std::io::{self, Write};

pub fn run() {
    println!("三目並べ");
    println!("入力するマスを入力してください");
    println!("-------------- -------------------------------");
    println!("| left top    | center top    | right top    |");
    println!("-------------- --------------- ---------------");
    println!("| left center | center        | right center |");
    println!("-------------- --------------- ---------------");
    println!("| left bottom | center bottom | right bottom |");
    println!("----------------------------------------------");
    print!("\n");

    let mut board = vec![CellValue::None; 9];
    minimax::print_board(&board);

    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let prompt = input.trim();
        if prompt == "exit" {
            break;
        }
        match input_to_index(prompt, &board) {
            Some(index) => {
                board[index] = CellValue::Player;
                minimax::print_board(&board);
                if minimax::is_player_win(&board, CellValue::Player) {
                    println!("おめでとうございます！完全勝利です！");
                    break;
                }
                if minimax::none_cell_not_exists(&board) {
                    println!(
                        "引き分けですかね。対戦ありがとうございました。"
                    );
                    break;
                }

                println!("考え中....");
                let enemy_move = minimax::best_move(&board);
                if enemy_move.is_none() {
                    println!(
                        "すみません。こちらの事情で申し訳ないのですが、投了とさせていただきます。"
                    );
                    break;
                }
                board[enemy_move.unwrap()] = CellValue::Enemy;
                minimax::print_board(&board);
                if minimax::is_player_win(&board, CellValue::Enemy) {
                    println!("すみません。AIの勝利です。");
                    break;
                }
                if minimax::none_cell_not_exists(&board) {
                    println!(
                        "これで引き分けですかね。対戦ありがとうございました。"
                    );
                    break;
                }
            }
            None => {
                println!("え？");
            }
        }
    }
}

fn input_to_index(input: &str, board: &Vec<CellValue>) -> Option<usize> {
    let parsed_input = match input {
        "left top" => Some(0),
        "center top" => Some(1),
        "right top" => Some(2),
        "left center" => Some(3),
        "center" => Some(4),
        "right center" => Some(5),
        "left bottom" => Some(6),
        "center bottom" => Some(7),
        "right bottom" => Some(8),
        _ => None,
    };
    if parsed_input.is_none() {
        return parsed_input;
    }
    let index = parsed_input.unwrap();
    if board[index] != CellValue::None {
        return None;
    }
    parsed_input
}
