extern crate rand;
use rand::Rng;
use std::convert::TryInto;

struct Queens {
    queens: Vec<u32>,
}

struct Board {
    board: Vec<Vec<bool>>,
}

impl Queens {
    fn generate_queens(queen_size: u32) -> Queens {
        let mut my_board = Vec::new();
        for i in 0..queen_size {
            my_board.push(rand::thread_rng().gen_range(0, queen_size));
        }

        return Queens { queens: my_board };
    }
    fn to_board(&self) -> Board {
        let mut board = Vec::new();
        for queen in &self.queens {
            let mut inner_queen = Vec::new();
            let mut index = 0u32;
            for i in 0..self.queens.len() {
                inner_queen.push(queen == &index);
                println!("{} - {}", queen, &index);
                index += 1;
            }
            board.push(inner_queen);
        }
        return Board { board: board };
    }
}

impl Board {
    fn print(&self) {
        for v1 in &self.board {
            for &b in v1 {
                if (b) {
                    print!("O");
                } else {
                    print!("X");
                }
            }
            println!();
        }
    }
}

fn main() {
    let _n = 8i32;
    println!("Input amount of queen");
    Queens::generate_queens(8).to_board().print();
}
