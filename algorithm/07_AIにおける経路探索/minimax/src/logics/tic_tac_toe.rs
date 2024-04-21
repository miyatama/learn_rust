use super::minimax::CellValue;
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
    print_board(&board);

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
        match input_to_index(prompt) {
            Some(index) => {
                board[index] = CellValue::Player;
                // TODO call minimax logic
                print_board(&board);
            }
            None => {
                println!("Ha?");
            }
        }
    }
}

fn print_board(board: &Vec<CellValue>) {
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

fn input_to_index(input: &str) -> Option<usize> {
    match input {
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
    }
}
