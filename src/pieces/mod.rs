use crate::board::Board;

use crate::common::Color;

use crate::positions::Position;

mod bishop;
mod diagonal_movement;
mod king;
mod knight;
mod parallel_movement;
mod pawn;
mod queen;
mod rook;

type StepChecker = Box<dyn Fn(&Piece, &Position, &Board) -> bool>;

#[derive(Debug, Eq, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position,
    pub movement: Box<dyn PieceMovement>,
    moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color, position: Position) -> Piece {
        let movement: Box<dyn PieceMovement> = match piece_type {
            PieceType::Pawn => Box::new(pawn::Pawn::new()),
            PieceType::Rook => Box::new(rook::Rook::new()),
            PieceType::Knight => Box::new(knight::Knight::new()),
            PieceType::Bishop => Box::new(bishop::Bishop::new()),
            PieceType::Queen => Box::new(queen::Queen::new()),
            PieceType::King => Box::new(king::King::new()),
        };
        Piece {
            piece_type,
            color,
            position,
            movement,
            moved: false,
        }
    }

    pub fn mark_as_moved(&mut self) {
        self.moved = true;
    }

    pub fn icon(&self) -> char {
        let unicode = match self.piece_type {
            PieceType::Pawn => 0x2659,
            PieceType::Knight => 0x2658,
            PieceType::Bishop => 0x2657,
            PieceType::Rook => 0x2656,
            PieceType::Queen => 0x2655,
            PieceType::King => 0x2654,
        };
        std::char::from_u32(if self.color == Color::Black {
            unicode + 6
        } else {
            unicode
        })
        .unwrap_or(' ')
    }
}

pub trait PieceMovement {
    fn new() -> Self
    where
        Self: Sized;
    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool;
    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker>;
    fn can_move(&self, piece: &Piece, new_position: &Position, board: &Board) -> bool {
        let diff = new_position - &piece.position;
        self.step_checker(diff)
            .map(|f| f(piece, new_position, board))
            .unwrap_or(false)
    }
}

fn verify_check_by_pos(piece: &Piece, other: Position, board: &Board) -> bool {
    if let Some(other_piece) = board.cell(&other) {
        verify_check(piece, other_piece)
    } else {
        false
    }
}

fn verify_check(piece: &Piece, other: &Piece) -> bool {
    other.color != piece.color && other.piece_type == PieceType::King
}

fn dest_is_same_color(piece: &Piece, new_position: &Position, board: &Board) -> bool {
    match board.cell(new_position) {
        None => false,
        Some(Piece { color, .. }) => *color == piece.color,
    }
}

fn path_is_empty(path: &[Position], board: &Board) -> bool {
    path[1..(path.len() - 1)]
        .iter()
        .all(|p| board.cell(&p).is_none())
}

fn find_check_state_in_path(path: &[Position], piece: &Piece, board: &Board) -> bool {
    path.iter()
        .skip(1)
        .find_map(|pos| board.cell(&pos).as_ref())
        .filter(|other| verify_check(piece, other))
        .is_some()
}
