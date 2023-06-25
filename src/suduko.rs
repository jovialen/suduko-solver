//! The game of Suduko.
//!
//! Suduko is a grid-based logic game on a 9x9 grid. The goal of the game is to
//! fill every 3x3 subgrid, row, and column with every number from 1-9 without
//! any overlap. The game begins with some of the numbers revealed, and ends
//! when all remaining cells in the grid have been filled out according to the
//! games rules.

/// The storage value for a cell on the Suduko grid.
///
/// For cells with set values, this is [`Some`]. If the cell is empty, this value
/// is [`None`].
pub type Cell = Option<u8>;

/// A game of suduko.
pub trait Suduko {
    /// Get the number on a cell on the grid.
    fn get(&self, i: usize) -> Cell;
    /// Set the number on a cell on the grid, or clear it.
    fn set(&mut self, i: usize, num: Cell);
    /// Get all the cells on the grid.
    fn cells(&self) -> &[Cell];
    /// Get a mutable refrence to all the cells on the grid.
    fn cells_mut(&mut self) -> &mut [Cell];
}

/// Standard game of Suduko.
///
/// This variation of the game is the standard one played. It is on a 9x9 grid
/// where the value of the cells have to be unique on the row and column, as
/// well as in one of the nine 3x3 subgrids.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Standard {
    cells: [Cell; 9 * 9],
}

impl Standard {
    pub fn new() -> Self {
        Self {
            cells: [None; 9 * 9],
        }
    }
}

impl Default for Standard {
    fn default() -> Self {
        Self::new()
    }
}

impl Suduko for Standard {
    fn get(&self, i: usize) -> Cell {
        self.cells[i]
    }

    fn set(&mut self, i: usize, num: Cell) {
        self.cells[i] = num;
    }

    fn cells(&self) -> &[Cell] {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut [Cell] {
        &mut self.cells
    }
}
