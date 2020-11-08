use crate::pieces::Piece;
use crate::positions;

#[derive(Debug)]
pub enum ChessError {
    InvalidPosition(String),
    InvalidMove(positions::Position, positions::Position),
    InvalidInput(String),
    EmptyCell(positions::Position),
    WrongColor(positions::Position),
}

pub enum StepResultData {
    Moved,
    Eaten(Piece),
    Check(Color),
}

pub type ChessResult<T> = std::result::Result<T, ChessError>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl std::convert::From<std::io::Error> for ChessError {
    fn from(io_error: std::io::Error) -> Self {
        ChessError::InvalidInput(io_error.to_string())
    }
}

impl std::convert::From<protobuf::ProtobufError> for ChessError {
    fn from(error: protobuf::ProtobufError) -> Self {
        ChessError::InvalidInput(error.to_string())
    }
}

impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use crate::common::ChessError::*;
        match self {
            InvalidPosition(pos) => write!(f, "Wrong position: {}", pos),
            InvalidMove(from, to) => write!(f, "Can't move from {} to {}", from, to),
            InvalidInput(input) => write!(f, "Invalid input: {}", input),
            EmptyCell(pos) => write!(f, "No piece on {}", pos),
            WrongColor(pos) => write!(f, "Wrong color on {}", pos),
        }
    }
}

impl std::error::Error for ChessError {}

impl std::ops::Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
