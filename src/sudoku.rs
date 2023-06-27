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
use std::str::FromStr;

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

/// Standard game of Suduko.
///
/// This variation of the game is the standard one played. It is on a 9x9 grid
/// where the value of the cells have to be unique on the row and column, as
/// well as in one of the nine 3x3 subgrids.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandardSudoku {
    cells: [Cell; 9 * 9],
}

impl StandardSudoku {
    pub fn new() -> Self {
        Self {
            cells: [None; 9 * 9],
        }
    }

    fn grid(&self, i: usize) -> Vec<Cell> {
        let row = i / 3;
        let col = i % 3;

        self.cells
            .chunks_exact(3)
            .skip(row * 9 + col)
            .step_by(3)
            .take(3)
            .flatten()
            .copied()
            .collect()
    }
}

impl Default for StandardSudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku for StandardSudoku {
    fn get(&self, i: usize) -> Cell {
        self.cells[i]
    }

    fn set(&mut self, i: usize, num: Cell) {
        if let Some(num) = num {
            if !self.cell_values().contains(&num) {
                panic!("{num} is not a valid value for this cell");
            }
        }

        self.cells[i] = num;
    }

    fn cells(&self) -> &[Cell] {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut [Cell] {
        &mut self.cells
    }

    fn cell_values(&mut self) -> RangeInclusive<u8> {
        1..=9
    }

    fn rows(&self) -> Vec<Vec<Cell>> {
        self.cells
            .chunks_exact(9)
            .map(|c| c.into_iter().copied().collect::<Vec<_>>())
            .collect()
    }

    fn columns(&self) -> Vec<Vec<Cell>> {
        [
            self.cells.into_iter().skip(0).step_by(9).collect(),
            self.cells.into_iter().skip(1).step_by(9).collect(),
            self.cells.into_iter().skip(2).step_by(9).collect(),
            self.cells.into_iter().skip(3).step_by(9).collect(),
            self.cells.into_iter().skip(4).step_by(9).collect(),
            self.cells.into_iter().skip(5).step_by(9).collect(),
            self.cells.into_iter().skip(6).step_by(9).collect(),
            self.cells.into_iter().skip(7).step_by(9).collect(),
            self.cells.into_iter().skip(8).step_by(9).collect(),
        ]
        .into()
    }

    fn grids(&self) -> Vec<Vec<Cell>> {
        (0..9).map(|i| self.grid(i)).collect()
    }

    fn groups_of(&self, i: usize) -> Vec<Vec<Cell>> {
        let row = i / 9;
        let col = i % 9;
        let group = (row / 3) * 3 + (col / 3);

        [
            self.cells.into_iter().skip(row * 9).take(9).collect(),
            self.cells.into_iter().skip(col).step_by(9).collect(),
            self.grid(group),
        ]
        .into()
    }
}

impl Display for StandardSudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .cells
                .iter()
                .map(|c| match c {
                    Some(digit) => ('0' as u8 + *digit) as char,
                    None => ' ',
                })
                .collect::<String>(),
        )
    }
}

impl FromStr for StandardSudoku {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .chars()
            .into_iter()
            .filter_map(|c| match c {
                // 1 to 9 become Some(1..9), ' ' becomes None
                '1'..='9' | ' ' => Some(c.to_digit(10).map(|d| d as u8)),
                _ => None,
            })
            .collect::<Vec<Cell>>();

        let mut cells = [None; 9 * 9];
        if v.len() != cells.len() {
            return Err("invalid length");
        }

        cells.copy_from_slice(&v[..]);

        Ok(Self { cells })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_parse_str() {
        let suduko = StandardSudoku::from_str(
            "   357891
35    7  
      5  
  5 4    
 7 98  5 
  35 62 8
  8    72
   42 18 
 92 18   ",
        );

        assert!(suduko.is_ok());
        let suduko = suduko.unwrap();

        assert_eq!(suduko.get(0), None);
        assert_eq!(suduko.get(3), Some(3));
        assert_eq!(suduko.get(10), Some(5));

        let suduko = StandardSudoku::from_str(
            "   35789135    7        5    5 4     7 98  5   35 62 8  8    72   42 18  92 18   ",
        );

        assert!(suduko.is_ok());
        let suduko = suduko.unwrap();

        assert_eq!(suduko.get(0), None);
        assert_eq!(suduko.get(3), Some(3));
        assert_eq!(suduko.get(10), Some(5));

        assert_eq!(
            suduko.to_string(),
            "   35789135    7        5    5 4     7 98  5   35 62 8  8    72   42 18  92 18   ",
        )
    }

    #[test]
    fn groups() {
        let suduko = StandardSudoku::from_str(
            "1234567892        3        4        5        6        7        8        987654321",
        )
        .unwrap();

        assert_eq!(
            suduko.rows()[0],
            (1..=9).map(|i| Some(i)).collect::<Vec<_>>()
        );
        assert_eq!(
            suduko.rows()[8],
            (1..=9).rev().map(|i| Some(i)).collect::<Vec<_>>()
        );

        assert_eq!(
            suduko.columns()[0],
            (1..=9).map(|i| Some(i)).collect::<Vec<_>>()
        );

        assert_eq!(
            suduko.grids()[0],
            Vec::from([
                Some(1),
                Some(2),
                Some(3),
                Some(2),
                None,
                None,
                Some(3),
                None,
                None
            ])
        );

        assert_eq!(
            suduko.grids()[8],
            Vec::from([
                None,
                None,
                None,
                None,
                None,
                None,
                Some(3),
                Some(2),
                Some(1),
            ])
        )
    }

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
