//! Suduko solver
//!
//! Suduko is a grid-based logic game on a 9x9 grid. The goal of the game is to
//! fill every 3x3 subgrid, row, and column with every number from 1-9 without
//! any overlap. The game begins with some of the numbers revealed, and ends
//! when all remaining cells in the grid have been filled out according to the
//! games rules.

mod suduko;
use suduko::Standard;

fn main() {
    let _game = Standard::default();
}
