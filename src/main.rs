//! Suduko solver

mod suduko;

use std::str::FromStr;
use suduko::{Standard, Suduko};

fn main() {
    let mut game = Standard::from_str(
        "  3  7  2  15  79  9      4        9 1   436   5 8    3  4           2   6   317 ",
    )
    .unwrap();

    let result = game.solve();
    println!("{result:?}");
    println!("{game}");
}
