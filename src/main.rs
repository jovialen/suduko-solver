//! Suduko solver

use std::str::FromStr;
use sudoku_solver::variants::HyperSudoku;
use sudoku_solver::{Sudoku, StandardSudoku};

fn main() {
    let mut game = HyperSudoku::from_str("       1   2    34    51        65   7 3   8   3          8    58    9  69       ").unwrap();

    let result = game.solve();
    println!("{result:?}");
    println!("{game}");
}
