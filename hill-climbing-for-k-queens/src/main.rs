extern crate rand;
use rand::Rng;

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
}
