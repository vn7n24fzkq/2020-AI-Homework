extern crate rand;
use rand::Rng;
use std::convert::TryInto;
use std::io;

struct Queens {
    queens: Vec<i32>,
}

struct Board {
    board: Vec<Vec<bool>>,
}

impl Queens {
    fn generate_queens(queen_size: i32) -> Queens {
        let mut my_board = Vec::new();
        for _i in 0..queen_size {
            my_board.push(rand::thread_rng().gen_range(0, queen_size));
        }

        return Queens { queens: my_board };
    }
    fn to_board(&self) -> Board {
        let mut board = Vec::new();
        for queen in &self.queens {
            let mut inner_queen = Vec::new();
            let mut index = 0i32;
            for _i in 0..self.queens.len() {
                inner_queen.push(queen == &index);
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
                if b {
                    print!("O");
                } else {
                    print!("X");
                }
            }
            println!();
        }
    }
}

fn run(size: i32) {
    let mut queens = Queens::generate_queens(size);
    let mut restart_time = 0;
    while get_heuristi_cost(&queens) != 0 {
        queens = next_state(&queens);
        if queens.queens.len() == 0 {
            queens = Queens::generate_queens(size);
            restart_time += 1;
        }
    }
    queens.to_board().print();
    println!("reset time : {}", restart_time);
}

fn next_state(queens: &Queens) -> Queens {
    let cost = get_heuristi_cost(queens);
    let temp_queens = get_new_queens(queens);
    if get_heuristi_cost(&temp_queens) < cost {
        return temp_queens;
    }
    return Queens::generate_queens(0);
}

fn get_new_queens(queens: &Queens) -> Queens {
    let mut old_queens = Queens {
        queens: queens.queens.clone(),
    };
    let mut new_queens = Queens::generate_queens(0);
    let mut cost = get_heuristi_cost(&old_queens);
    for i in 0..old_queens.queens.len() {
        new_queens.queens.clear();
        new_queens.queens.push(old_queens.queens[i]);
        for j in 0..old_queens.queens.len() {
            let jj: i32 = j.try_into().unwrap();
            old_queens.queens[i] = jj;
            let new_cost = get_heuristi_cost(&old_queens);
            if cost == new_cost {
                new_queens.queens.push(jj);
            } else if new_cost < cost {
                cost = new_cost;
                new_queens.queens.clear();
                new_queens.queens.push(jj);
            }
        }
        old_queens.queens[i] =
            new_queens.queens[rand::thread_rng().gen_range(0, new_queens.queens.len())];
    }
    return old_queens;
}

fn get_heuristi_cost(queens: &Queens) -> i32 {
    let mut cost = 0;
    for i in 0..(queens.queens.len() - 1) {
        for j in i + 1..queens.queens.len() {
            if queens.queens[i] == queens.queens[j] {
                cost += 1;
            } else if (queens.queens[i] - queens.queens[j]).abs() == (i as i32 - j as i32).abs() {
                cost += 1;
            }
        }
    }
    return cost;
}

fn main() {
    println!("Input amount of queen");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: i32 = input.trim().parse().unwrap();
    run(_n);
}
