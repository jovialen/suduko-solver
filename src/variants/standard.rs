use crate::sudoku::{Cell, Sudoku};
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

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
}
