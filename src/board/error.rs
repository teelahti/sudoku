#[derive(Debug)]
pub enum BoardError {
    InvalidLength(usize),
    InvalidDigit(u8),
}

impl std::fmt::Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BoardError::InvalidLength(d) => write!(f, "Invalid board data length {} != 81", d),
            BoardError::InvalidDigit(d) => write!(f, "Invalid number '{}' on board", d),
        }
    }
}
impl std::error::Error for BoardError {}

#[derive(Debug)]
pub enum BoardParseError {
    EmptyInput,
    NotEnoughCells,
    TooManyCells,
    InvalidBoard(BoardError),
}

impl std::fmt::Display for BoardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BoardParseError::EmptyInput => write!(f, "Empty input"),
            BoardParseError::NotEnoughCells => {
                write!(f, "Not enough cells (numbers or blanks) in input")
            }
            BoardParseError::TooManyCells => {
                write!(f, "Too many cells (numbers or blanks) in input")
            }
            BoardParseError::InvalidBoard(e) => {
                write!(f, "The input board is in invalid state: {}", e)
            }
        }
    }
}
impl std::error::Error for BoardParseError {}
