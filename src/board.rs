mod error;
use core::panic;

use error::*;

// Use matrix library for easier slicing. Would not be too bad of a rewrite to do without.
type Grid = nalgebra::SMatrix<u8, 9, 9>;
pub const EMPTY: u8 = 0;

#[derive(Clone)]
pub struct Board {
    b: Grid,
}

impl Board {
    pub fn new(b: Vec<u8>) -> Result<Self, BoardError> {
        if b.len() != 9 * 9 {
            return Err(BoardError::InvalidLength(b.len()));
        }

        for d in b.iter() {
            if *d > 9 {
                return Err(BoardError::InvalidDigit(*d));
            }
        }

        Ok(Board {
            b: Grid::from_row_iterator(b.into_iter()),
        })
    }

    pub fn get(self: &Self, y: usize, x: usize) -> Option<&u8> {
        self.b.get((y, x))
    }

    // Sets the value. Panics if out of bounds or value is too big.
    // TODO: Using Errors would be nicer
    pub fn set(self: &mut Self, y: usize, x: usize, d: u8) {
        if d > 9 {
            panic!("Invalid sudoku cell value {}", d);
        }
        self.b[(y, x)] = d;
    }

    // Evaluates all the possible values that can be set in given index and returns them.
    pub fn possible_inputs(self: &Self, y: usize, x: usize) -> impl Iterator<Item = u8> + use<> {
        // Takes all the zeros from row, col, and house, and return the ones
        // that are zero on all of them (i.e. not used)
        let (s1, s2, s3) = self.sums(y, x);
        const ZERO: u16 = 0b1111_1110_0000_0000;

        let possible = !(s1 ^ ZERO) & !(s2 ^ ZERO) & !(s3 ^ ZERO);

        // Our bitmap is 0-based but numbers are 1-based
        (0..9u8)
            .filter(move |n| possible & (1 << n) != 0)
            .map(|n| n + 1)
    }

    // Calculate row, column, and house sums in that order
    fn sums(self: &Self, y: usize, x: usize) -> (u16, u16, u16) {
        let (hy, hx): (usize, usize) = (y / 3, x / 3);
        let sfn = |acc, d: u8| if d == 0 { acc } else { acc | 1 << (d - 1) };
        (
            self.b.row(y).fold(0, sfn),
            self.b.column(x).fold(0, sfn),
            self.b
                .index((hy * 3..hy * 3 + 3, hx * 3..hx * 3 + 3))
                .fold(0, sfn),
        )
    }
}

impl std::str::FromStr for Board {
    type Err = BoardParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(BoardParseError::EmptyInput);
        }
        // Skip everything but digits and blanks
        let v: Vec<u8> = trimmed
            .chars()
            .filter_map(|c| {
                // Blank to 0
                if c == '_' {
                    return Some(0);
                }
                // Char digit to u8
                (c as u8 >= b'0' && c as u8 <= b'9').then_some(c as u8 - b'0')
            })
            .collect();
        match v.len() {
            n if n < 9 * 9 => Err(BoardParseError::NotEnoughCells),
            n if n > 9 * 9 => Err(BoardParseError::TooManyCells),
            _ => Self::new(v).map_err(|e| BoardParseError::InvalidBoard(e)),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, r) in self.b.row_iter().enumerate() {
            for (j, d) in r.iter().enumerate() {
                if *d == 0 {
                    _ = write!(f, "  ");
                } else {
                    _ = write!(f, "{} ", d);
                }
                if j == 2 || j == 5 {
                    _ = write!(f, "| ");
                }
            }

            _ = write!(f, "\n");

            if i == 2 || i == 5 {
                _ = write!(f, "- - - + - - - + - - - \n");
            }
        }

        // TODO: Calculate stats and add
        write!(f, "\n")
    }
}

// #[test]

// // TODO: Create better test func name
// macro_rules! test_from_str_fail {
//     ($name:ident, $input:expr) => {
//         #[test]
//         #[should_panic]
//         fn $name() {
//             Board::from_str($input);
//         }
//     };
// }

// test_from_str_fail!(empty, "");
// test_from_str_fail!(no_numbers, "aaaaaaaaaa");
