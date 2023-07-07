use crate::sudoku::{Cell, Sudoku};
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HyperSudoku {
    cells: [Cell; 9 * 9],
}

impl HyperSudoku {
    pub fn new() -> Self {
        Self {
            cells: [None; 9 * 9],
        }
    }

    fn grid(&self, i: usize) -> Vec<Cell> {
        let row = i / 3;
        let col = i % 3;

		let offset = match i {
			0..=8 => (row * 9 * 3) + col * 3,
			9 => 9 + 1,
			10 => 9 + 5,
			11 => 9 * 5 + 1,
			12 => 9 * 5 + 5,
			_ => unreachable!("{i}"),
		};

        let indices = (offset..(offset + 3)).chain((offset + 9)..(offset + 9 + 3)).chain((offset + 18)..(offset + 18 + 3));

        indices.map(|i| self.cells[i]).collect()
    }
}

impl Default for HyperSudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku for HyperSudoku {
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
		(0..9).map(|i| self.cells.into_iter().skip(i).step_by(9).collect()).collect()
    }

    fn grids(&self) -> Vec<Vec<Cell>> {
        (0..13).map(|i| self.grid(i)).collect()
    }

    fn groups_of(&self, i: usize) -> Vec<Vec<Cell>> {
		let mut v = Vec::with_capacity(4);
		
        let row = i / 9;
        let col = i % 9;
        let group = (row / 3) * 3 + (col / 3);

		v.push(self.cells.into_iter().skip(row * 9).take(9).collect());
		v.push(self.cells.into_iter().skip(col).step_by(9).collect());
		v.push(self.grid(group));

		match (row, col) {
			(1..=3, 1..=3) => v.push(self.grid(9)),
			(1..=3, 5..=7) => v.push(self.grid(10)),
			(5..=7, 1..=3) => v.push(self.grid(11)),
			(5..=7, 5..=7) => v.push(self.grid(12)),
            _ => (),
		}

		v
    }
}

impl Display for HyperSudoku {
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

impl FromStr for HyperSudoku {
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
    fn groups() {
        let game = HyperSudoku::from_str("       1   2    34    51        65   7 3   8   3          8    58    9  69       ").unwrap();

        assert_eq!(
            game.grids()[0],
            Vec::from([
                None,
                None,
                None,
                None,
                None,
                Some(2),
                None,
                None,
                None,
            ])
        );

        assert_eq!(
            game.grids()[1],
            Vec::from([
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(5),
                Some(1),
            ]),
        );

        assert_eq!(
            game.grids()[9],
            Vec::from([
                None,
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ])
        );

        assert_eq!(
            game.grids()[10],
            Vec::from([
                None,
                None,
                Some(3),
                Some(1),
                None,
                None,
                Some(6),
                Some(5),
                None,
            ])
        );

        assert_eq!(
            game.grids()[11],
            Vec::from([
                None,
                Some(3),
                None,
                None,
                None,
                None,
                Some(8),
                None,
                None,
            ])
        );

        assert_eq!(
            game.grids()[12],
            Vec::from([
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(9),
                None,
            ])
        );
    }

    #[test]
    fn groups_of() {
        let game = HyperSudoku::from_str("       1   2    34    51        65   7 3   8   3          8    58    9  69       ").unwrap();

        assert_eq!(game.groups_of(0).len(), 3);
        assert_eq!(game.groups_of(9 + 3)[3], game.grid(9));
        assert_eq!(game.groups_of(9 + 4).len(), 3);
        assert_eq!(game.groups_of(9 + 5)[3], game.grid(10));
        assert_eq!(game.groups_of(9 * 2 + 3)[3], game.grid(9));
        assert_eq!(game.groups_of(5)[2], game.grid(1));
    }
}
