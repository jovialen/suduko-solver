//! Suduko solver

mod suduko;

use std::str::FromStr;
use suduko::Standard;

fn main() {
    let _game = Standard::from_str(
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
}
