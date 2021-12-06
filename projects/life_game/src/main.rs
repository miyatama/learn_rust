use clap::{Arg, App};
use rand::prelude::*;
use std::{thread, time};

fn main() {
    let matches = App::new("life geme of slones")
        .version("1.0.0")
        .arg(
            Arg::with_name("generation")
            .short("g")
            .long("generation")
            .value_name("GENERATION")
            .help("set generation")
            .takes_value(true))
        .get_matches();

    let mut generation = 0;
    if let Some(value) = matches.value_of("generation") {
        generation = get_generation_args(value);
    } else {
        println!("--generation <GENERATION> wasn't used...");
        return;
    }

    println!("generation is {}", generation);
    let mut world = World::generate(100,50);
    // show generation
    world.print_world();

    let mut i = 0;
    while i < generation {
        // calc next generation
        world = world.calc_next_generation();
        // show generation
        world.print_world();

        thread::sleep(time::Duration::from_millis(300));

        i = i + 1;
    }    
}

fn get_generation_args(value: &str) -> u32 {
    value.parse().unwrap()
}

struct World {
    cols: usize,
    rows: usize,
    generation: u32,
    cells: Vec<Vec<bool>>,
}

impl World {
    fn generate(cols: usize, rows: usize) -> Self {
        let mut world = Self {
            cols,
            rows,
            generation: 0,
            cells: vec![vec![false; cols]; rows],
        };

        let mut i = 0;
        while i < rows {
            let mut j = 0;
            while j < cols {
                world.cells[i][j] = rand::random();
                j = j + 1;
            }    
            i = i + 1;
        }
        world
    }

    fn calc_next_generation(&mut self) -> Self 
    {
        let mut i: usize = 0;
        while i < self.rows {
            let mut j: usize = 0;
            while j < self.cols {
                let neibors = World::get_neibors(&self.cells, i, j);
                self.cells[i][j] = World::next_action(self.cells[i][j], neibors);
                j = j + 1;
            }
            i = i + 1;
        }

        World {
            cols: self.cols,
            rows: self.rows,
            generation: self.generation + 1,
            cells: self.cells.clone(),
        }
    }

    fn get_neibors(cells: &Vec<Vec<bool>>, row: usize, col: usize) -> Vec<bool> {
        let mut neibors = vec![];    
        let min_row = if row < 1 { row } else { row - 1} ;
        let max_row = if row >= (cells.len() - 1) { row } else { row + 1};
        let min_col = if col < 1 { col }else{ col - 1};
        let max_col = if col >= (cells[0].len() - 1) { col } else { col + 1};
        let mut i = min_row;
        while i <= max_row {
            let mut j = min_col;
            while j <= max_col {
                if i == row && col == j {
                    j = j + 1;
                    continue;
                }
                neibors.push(cells[i][j]);
                j = j + 1;
            }
            i = i + 1;
        }
        neibors
    }

    fn next_action(cell: bool, neibors: Vec<bool>) -> bool{
        let mut liveCellCount = 0;
        for neibor in neibors {
            if neibor {
                liveCellCount = liveCellCount + 1;
            }
        }
        let mut result = false;

        // underpopulation
        if cell && liveCellCount < 2 {
            result = false;
        }        

        // survive
        if cell && (liveCellCount == 2 || liveCellCount == 3) {
            result = true;
        }        

        // overpopulation
        if cell && liveCellCount >= 3 {
            result = false;
        }        

        // reproduction
        if !cell && liveCellCount >= 3 {
            result = true;
        }
    
        result
    }

    fn print_world(&self) {
        print!("{}[2J", 27 as char);
        println!("----- generation: {}", self.generation);
        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                if self.cells[i][j] {
                    print!("x");
                }else {
                    print!(" ");
                }
                j = j + 1;
            }
            print!("\n");
            i = i + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_get_neibors_left_top_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[0][1] = true;
        cells[1][0] = true;
        cells[1][1] = true;
        let neibors = World::get_neibors(&cells, 0, 0);       
        assert_eq!(3, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2]);
    }

    #[test]
    fn test_world_get_neibors_left_bottom_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[8][0] = true;
        cells[8][1] = true;
        cells[9][1] = true;
        let neibors = World::get_neibors(&cells, 9, 0);       
        assert_eq!(3, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2]);
    }

    #[test]
    fn test_world_get_neibors_right_top_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[0][8] = true;
        cells[1][8] = true;
        cells[1][9] = true;
        let neibors = World::get_neibors(&cells, 0, 9);       
        assert_eq!(3, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2]);
    }

    #[test]
    fn test_world_get_neibors_right_bottom_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[8][9] = true;
        cells[8][8] = true;
        cells[9][8] = true;
        let neibors = World::get_neibors(&cells, 9, 9);       
        assert_eq!(3, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2]);
    }

    #[test]
    fn test_world_get_neibors_top_center_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[0][4] = true;
        cells[0][6] = true;
        cells[1][4] = true;
        cells[1][5] = true;
        cells[1][6] = true;
        let neibors = World::get_neibors(&cells, 0, 5);       
        assert_eq!(5, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2] && neibors[3] && neibors[4]);
    }

    #[test]
    fn test_world_get_neibors_bottom_center_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[8][4] = true;
        cells[8][5] = true;
        cells[8][6] = true;
        cells[9][4] = true;
        cells[9][6] = true;
        let neibors = World::get_neibors(&cells, 9, 5);       
        assert_eq!(5, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2] && neibors[3] && neibors[4]);
    }

    #[test]
    fn test_world_get_neibors_left_center_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[4][0] = true;
        cells[4][1] = true;
        cells[5][1] = true;
        cells[6][0] = true;
        cells[6][1] = true;
        let neibors = World::get_neibors(&cells, 5, 0);       
        assert_eq!(5, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2] && neibors[3] && neibors[4]);
    }

    #[test]
    fn test_world_get_neibors_right_center_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[4][8] = true;
        cells[4][9] = true;
        cells[5][8] = true;
        cells[6][8] = true;
        cells[6][9] = true;
        let neibors = World::get_neibors(&cells, 5, 9);       
        assert_eq!(5, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2] && neibors[3] && neibors[4]);
    }

    #[test]
    fn test_world_get_neibors_mid_center_cell() {
        let mut cells = vec![vec![false; 10]; 10];
        cells[4][4] = true;
        cells[4][5] = true;
        cells[4][6] = true;
        cells[5][4] = true;
        cells[5][6] = true;
        cells[6][4] = true;
        cells[6][5] = true;
        cells[6][6] = true;
        let neibors = World::get_neibors(&cells, 5, 5);       
        assert_eq!(8, neibors.len());
        assert!(neibors[0] && neibors[1] && neibors[2] && neibors[3] && neibors[4] && neibors[5] && neibors[6] && neibors[7]);
    }

    #[test]
    fn test_world_next_action_underpopulation() {
        let cell = true;
        let mut neibors = vec![false; 8];
        neibors[0] = true;        
        let result = World::next_action(cell, neibors);
        assert!(!result);
    }

    #[test]
    fn test_world_next_action_survive() {
        let cell = true;
        let mut neibors = vec![false; 8];
        neibors[0] = true;
        neibors[1] = true;        
        assert!(World::next_action(cell, neibors));
    }
    
    #[test]
    fn test_world_next_action_overpopulation() {
        let cell = true;
        let mut neibors = vec![false; 8];
        neibors[0] = true;
        neibors[1] = true;        
        neibors[2] = true;        
        assert!(!World::next_action(cell, neibors));
    }
    
    #[test]
    fn test_world_next_action_reproduction() {
        let cell = false;
        let mut neibors = vec![false; 8];
        neibors[0] = true;
        neibors[1] = true;        
        neibors[2] = true;        
        assert!(World::next_action(cell, neibors));
    }
    
}