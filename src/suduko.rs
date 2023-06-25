//! The game of Suduko.
//!
//! Suduko is a grid-based logic game on a 9x9 grid. The goal of the game is to
//! fill every 3x3 subgrid, row, and column with every number from 1-9 without
//! any overlap. The game begins with some of the numbers revealed, and ends
//! when all remaining cells in the grid have been filled out according to the
//! games rules.

use std::fmt::Display;
use std::str::FromStr;

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

impl Display for Standard {
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

impl FromStr for Standard {
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
        let suduko = Standard::from_str(
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

        let suduko = Standard::from_str(
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
}
