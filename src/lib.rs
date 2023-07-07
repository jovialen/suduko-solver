//! A library for playing and solving the game of Sudoku.
//!
//! Sudoku is a numbers-based logic game centered around filling a 9x9 grid
//! with the numbers of 1 to 9 without repeating the same number on the same
//! row, column, or 3x3 sub-grid. The game is started with a partially filled
//! grid, which the player must use to place the remaining empty cells with the
//! correct digits, without ever modifying the initially revealed digits.
//!
//! The game also has some variants, like mini-sudoku where the grid is
//! smaller, or hyper-sudoku, where there are more sub-grids on the grid which
//! further constrains which digits you can place.

#![warn(missing_docs)]

pub mod sudoku;
pub mod variants;

pub use sudoku::*;
pub use variants::StandardSudoku;
