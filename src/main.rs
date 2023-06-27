//! Suduko solver

use std::str::FromStr;
use sudoku_solver::variants::MiniSudoku;
use sudoku_solver::Sudoku;

fn main() {
    let mut game = MiniSudoku::from_str("  5 642645 1  3 4  561 3 4 3 66    2").unwrap();

    let result = game.solve();
    println!("{result:?}");
    println!("{game}");
}
