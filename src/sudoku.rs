//! The game of Suduko.
//!
//! Suduko is a grid-based logic game on a 9x9 grid. The goal of the game is to
//! fill every 3x3 subgrid, row, and column with every number from 1-9 without
//! any overlap. The game begins with some of the numbers revealed, and ends
//! when all remaining cells in the grid have been filled out according to the
//! games rules.

use rustc_hash::FxHashSet;
use std::fmt::Display;
use std::ops::RangeInclusive;

/// The storage value for a cell on the Suduko grid.
///
/// For cells with set values, this is [`Some`]. If the cell is empty, this value
/// is [`None`].
pub type Cell = Option<u8>;

/// A game of suduko.
pub trait Sudoku: Sized + Display {
    /// Get the number on a cell on the grid.
    ///
    /// # Panics
    ///
    /// Panics if `i` is not within the bounds of the suduko.
    fn get(&self, i: usize) -> Cell;
    /// Set the number on a cell on the grid, or clear it.
    ///
    /// # Panics
    ///
    /// Panics if `i` is not within the bounds of the suduko or if `num` is not
    /// one of [`Self::cell_values`].
    fn set(&mut self, i: usize, num: Cell);
    /// Get all the cells on the grid.
    fn cells(&self) -> &[Cell];
    /// Get a mutable refrence to all the cells on the grid.
    fn cells_mut(&mut self) -> &mut [Cell];

    /// Get all possible valid values for the cells.
    fn cell_values(&mut self) -> RangeInclusive<u8>;

    /// Get all the rows.
    fn rows(&self) -> Vec<Vec<Cell>>;
    /// Get all the columns.
    fn columns(&self) -> Vec<Vec<Cell>>;
    /// Get all the subgrids.
    fn grids(&self) -> Vec<Vec<Cell>>;

    /// Get all groups a cell is part of.
    fn groups_of(&self, i: usize) -> Vec<Vec<Cell>>;

    /// Get all cell groups.
    ///
    /// This includes [`Self::rows`], [`Self::columns`] and [`Self::grids`].
    fn groups(&self) -> Vec<Vec<Cell>> {
        let mut v = Vec::new();
        v.append(&mut self.rows());
        v.append(&mut self.columns());
        v.append(&mut self.grids());
        v
    }

    /// Check if all cells in the suduko has been filled.
    fn filled(&self) -> bool {
        self.cells().iter().all(|c| c.is_some())
    }

    /// Check if all currently set cells are legal.
    fn legal(&self) -> bool {
        let groups = self.groups();
        groups.into_iter().all(|mut group| {
            // Check that all cells in group that are set are unique.
            group.sort();
            group.windows(2).all(|w| w[0] != w[1] || w[0].is_none())
        })
    }

    /// Check if the suduko has been solved.
    fn solved(&self) -> bool {
        let groups = self.groups();
        groups.into_iter().all(|mut group| {
            // Check that both all cells in group are set and that there are no
            // repeating values.
            group.sort();
            group[0] != None && group.windows(2).all(|w| w[0] != w[1])
        })
    }

    /// Solve the suduko
    fn solve(&mut self) -> Result<(), &'static str> {
        backtrack(self, 0)
    }
}

fn backtrack(suduko: &mut impl Sudoku, pos: usize) -> Result<(), &'static str> {
    if !suduko.legal() {
        return Err("cannot solve illegal position");
    }

    if pos >= suduko.cells().len() {
        return Ok(());
    }

    if suduko.get(pos).is_some() {
        return backtrack(suduko, pos + 1);
    }

    let illegal = FxHashSet::from_iter(suduko.groups_of(pos).into_iter().flatten());
    let possible = suduko
        .cell_values()
        .filter(|&value| !illegal.contains(&Some(value)));

    for value in possible {
        suduko.set(pos, Some(value));
        if let Ok(_) = backtrack(suduko, pos + 1) {
            return Ok(());
        }
    }

    suduko.set(pos, None);

    Err("suduko cannot be solved")
}

#[cfg(test)]
mod tests {
    use crate::variants::StandardSudoku;
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validation() {
        let suduko = StandardSudoku::from_str(
            "827154396965327148341689752593468271472513689618972435786235914154796823239841567",
        )
        .unwrap();
        assert!(suduko.filled());
        assert!(suduko.solved());

        let suduko = StandardSudoku::from_str(
            "227154396965327148341689752593468271472513689618972435786235914154796823239841567",
        )
        .unwrap();
        assert!(suduko.filled());
        assert!(!suduko.solved());

        let suduko = StandardSudoku::from_str(
            " 27154396965327148341689752593468271472513689618972435786235914154796823239841567",
        )
        .unwrap();
        assert!(!suduko.filled());
        assert!(!suduko.solved());
    }

    #[test]
    fn solve() {
        let mut suduko = StandardSudoku::from_str(
            "7 2 519  3 492 1      7 65 931      2    738 67 34  1949768 2 11   3         94 7",
        )
        .unwrap();
        assert!(suduko.solve().is_ok());
        assert_eq!(
            suduko.to_string(),
            "762851943354926178819473652931568724245197386678342519497685231126734895583219467"
        );
    }
}
