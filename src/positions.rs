pub use crate::common::ChessError;
use crate::common::ChessError::InvalidPosition;
use crate::proto::chess::Cell;
use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Horizontal {
    HorA = 0,
    HorB,
    HorC,
    HorD,
    HorE,
    HorF,
    HorG,
    HorH,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Vertical {
    Vert1 = 0,
    Vert2,
    Vert3,
    Vert4,
    Vert5,
    Vert6,
    Vert7,
    Vert8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub hor: Horizontal,
    pub vert: Vertical,
}

impl Position {
    pub fn from_proto(cell: &Cell) -> Option<Position> {
        cell.name.parse().ok()
    }

    pub fn at(hor: i8, vert: i8) -> Option<Position> {
        match (FromPrimitive::from_i8(hor), FromPrimitive::from_i8(vert)) {
            (Some(hor), Some(vert)) => Some(Position { hor, vert }),
            _ => None,
        }
    }

    pub fn transform(&self, hor_diff: i8, vert_diff: i8) -> Option<Position> {
        match (
            FromPrimitive::from_i8(self.hor as i8 + hor_diff),
            FromPrimitive::from_i8(self.vert as i8 + vert_diff),
        ) {
            (Some(hor), Some(vert)) => Some(Position { hor, vert }),
            _ => None,
        }
    }
}

impl std::ops::Sub for &Position {
    type Output = (i8, i8);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.hor as i8 - other.hor as i8,
            self.vert as i8 - other.vert as i8,
        )
    }
}

impl std::str::FromStr for Position {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hor = match s.bytes().nth(0) {
            Some(x @ b'A'..=b'H') => FromPrimitive::from_u8(x - b'A').unwrap(),
            Some(x @ b'a'..=b'h') => FromPrimitive::from_u8(x - b'a').unwrap(),
            _ => return Err(InvalidPosition(s.to_string())),
        };
        let vert = match s.bytes().nth(1) {
            Some(x @ b'1'..=b'8') => FromPrimitive::from_u8(x - b'1').unwrap(),
            _ => return Err(InvalidPosition(s.to_string())),
        };
        Ok(Position { hor, vert })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.hor as u8 + b'A') as char,
            self.vert as u8 + 1
        )
    }
}
