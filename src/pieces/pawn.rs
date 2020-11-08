use crate::pieces::Board;
use crate::pieces::Color::*;
use crate::pieces::Piece;
use crate::pieces::PieceMovement;
use crate::pieces::StepChecker;
use crate::Position;

pub struct Pawn;

impl PieceMovement for Pawn {
    fn new() -> Self {
        Pawn {}
    }

    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        let mult = if piece.color == White { 1 } else { -1 };
        if let Some(other) = piece.position.transform(-1, 1 * mult) {
            if super::verify_check_by_pos(piece, other, board) {
                return true;
            }
        }
        if let Some(other) = piece.position.transform(1, 1 * mult) {
            if super::verify_check_by_pos(piece, other, board) {
                return true;
            }
        }
        false
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        match diff {
            (0, 1) => Some(Box::new(check_up)),
            (0, 2) => Some(Box::new(check_long_up)),
            (-1, 1) => Some(Box::new(check_diagonal_up)),
            (1, 1) => Some(Box::new(check_diagonal_up)),
            (0, -1) => Some(Box::new(check_down)),
            (0, -2) => Some(Box::new(check_long_down)),
            (-1, -1) => Some(Box::new(check_diagonal_down)),
            (1, -1) => Some(Box::new(check_diagonal_down)),
            _ => None,
        }
    }
}

fn check_up(me: &Piece, new_position: &Position, board: &Board) -> bool {
    me.color == White && check_forward(me, board.cell(new_position))
}

fn check_long_up(me: &Piece, new_position: &Position, board: &Board) -> bool {
    me.color == White
        && check_long_forward(me, board.cell(new_position))
        && board
            .cell(&new_position.transform(0, -1).unwrap_or(me.position.clone()))
            .is_none()
}

fn check_diagonal_up(me: &Piece, new_position: &Position, board: &Board) -> bool {
    me.color == White && check_diagonal(me, board.cell(new_position))
}

fn check_down(me: &Piece, new_position: &Position, board: &Board) -> bool {
    me.color == Black && check_forward(me, board.cell(new_position))
}

fn check_long_down(me: &Piece, new_position: &Position, board: &Board) -> bool {
    me.color == Black
        && check_long_forward(me, board.cell(new_position))
        && board
            .cell(&new_position.transform(0, 1).unwrap_or(me.position.clone()))
            .is_none()
}

fn check_diagonal_down(me: &Piece, new_position: &Position, board: &Board) -> bool {
    me.color == Black && check_diagonal(me, board.cell(new_position))
}

fn check_forward(_: &Piece, dest: &Option<Piece>) -> bool {
    dest.is_none()
}

fn check_long_forward(me: &Piece, dest: &Option<Piece>) -> bool {
    !me.moved && dest.is_none()
}

fn check_diagonal(me: &Piece, dest: &Option<Piece>) -> bool {
    dest.as_ref()
        .map(|other| other.color != me.color)
        .unwrap_or(false)
}
