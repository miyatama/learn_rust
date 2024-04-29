use super::astar::{self, Evaluation};

pub fn run() {
    println!("8パズルを解きます。見るだけ見といてください。");
    print!("\n");

    let mut board = vec![8, 1, 3, 2, 4, 5, 0, 7, 6];
    astar::print_board(&board);

    let evaluators = vec![
        Evaluation::FairEvaluator,
        Evaluation::GoodEvaluator,
        Evaluation::WeakEvaluator,
        Evaluation::BadEvaluator,
    ];
    for i in 0..evaluators.len() {
        println!("{:?}で考え中...", evaluators[i]);
        match astar::think(&board, &evaluators[i]) {
            None => {
                println!("これは解けない問題です！");
            }
            Some(directions) => {
                println!("解けた！");
                astar::print_board(&board);
                for i in 0..directions.len() {
                    board = astar::apply_direction(&board, directions[i]);
                    println!("↓");
                    astar::print_board(&board);
                }
                if directions.len() <= 0 {
                    println!("が、解けたとはいってない！");
                }
                println!("な？");
            }
        }
    }
}
