use crate::suduko::{Cell, Suduko};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MiniSudoku {
    cells: [Cell; 6 * 6],
}

impl MiniSudoku {
    pub fn new() -> Self {
        Self {
            cells: [None; 6 * 6],
        }
    }

    fn row(&self, i: usize) -> Vec<Cell> {
        self.cells.into_iter().skip(i * 6).take(6).collect()
    }

    fn column(&self, i: usize) -> Vec<Cell> {
        self.cells.into_iter().skip(i).step_by(6).collect()
    }

    fn grid(&self, i: usize) -> Vec<Cell> {
        let row = i / 2;
        let col = i % 2;

        self.cells
            .chunks_exact(3)
            .skip(row * 4)
            .skip(col)
            .step_by(2)
            .take(2)
            .flatten()
            .copied()
            .collect()
    }
}

impl Suduko for MiniSudoku {
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

    fn cell_values(&mut self) -> std::ops::RangeInclusive<u8> {
        1..=6
    }

    fn rows(&self) -> Vec<Vec<Cell>> {
        (0..6).map(|i| self.row(i)).collect()
    }

    fn columns(&self) -> Vec<Vec<Cell>> {
        (0..6).map(|i| self.column(i)).collect()
    }

    fn grids(&self) -> Vec<Vec<Cell>> {
        (0..6).map(|i| self.grid(i)).collect()
    }

    fn groups_of(&self, i: usize) -> Vec<Vec<Cell>> {
        let row = i / 6;
        let col = i % 6;
        let group = (row / 2) * 2 + (col / 3);

        [self.row(row), self.column(col), self.grid(group)].into()
    }
}

impl Display for MiniSudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .cells
                .into_iter()
                .map(|c| match c {
                    Some(num) => ('0' as u8 + num) as char,
                    None => ' ',
                })
                .collect::<String>(),
        )
    }
}

impl FromStr for MiniSudoku {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .chars()
            .into_iter()
            .filter_map(|c| match c {
                '1'..='6' | ' ' => Some(c.to_digit(10).map(|n| n as u8)),
                _ => None,
            })
            .collect::<Vec<_>>();

        let mut cells = [None; 6 * 6];

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
    fn groups() {
        let game = MiniSudoku::from_str("  5 642645 1  3 4  561 3 4 3 66    2").unwrap();

        assert_eq!(game.row(0), [None, None, Some(5), None, Some(6), Some(4)]);
        assert_eq!(game.row(5), [Some(6), None, None, None, None, Some(2)]);
        assert_eq!(game.column(0), [None, Some(2), None, None, None, Some(6)]);
        assert_eq!(
            game.column(5),
            [Some(4), Some(1), None, Some(3), Some(6), Some(2)]
        );
        assert_eq!(
            game.grid(0),
            [None, None, Some(5), Some(2), Some(6), Some(4)]
        );
        assert_eq!(
            game.grid(1),
            [None, Some(6), Some(4), Some(5), None, Some(1)]
        );
        assert_eq!(game.grid(2), [None, None, Some(3), None, Some(5), Some(6)]);
        assert_eq!(game.grid(5), [Some(3), None, Some(6), None, None, Some(2)]);

        let groups_of = game.groups_of(15);
        assert_eq!(groups_of[0], game.row(2));
        assert_eq!(groups_of[1], game.column(3));
        assert_eq!(groups_of[2], game.grid(3));
    }
}
