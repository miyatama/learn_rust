use super::bfs;

pub fn run() {
    println!("8パズルを解きます。見るだけ見といてください。");
    print!("\n");

    let mut board = vec![8, 1, 3, 2, 4, 5, 0, 7, 6];
    bfs::print_board(&board);

    println!("考え中...");
    match bfs::think(&board) {
        None => {
            println!("これは解けない問題です！");
        }
        Some(directions) => {
            println!("解けた！");
            bfs::print_board(&board);
            for i in 0..directions.len() {
                board = bfs::apply_direction(&board, directions[i]);
                println!("↓");
                bfs::print_board(&board);
            }
            if directions.len() <= 0 {
                println!("が、解けたとはいってない！");
            }
            println!("な？");
        }
    }
}
