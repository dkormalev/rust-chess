use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::PieceMovement;
use crate::pieces::StepChecker;
use crate::Position;

pub struct King;

impl PieceMovement for King {
    fn new() -> Self {
        King {}
    }

    fn is_enemy_in_check(&self, _piece: &Piece, _board: &Board) -> bool {
        false
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        match diff {
            (0, 0) => None,
            (-1..=1, -1..=1) => Some(Box::new(check)),
            _ => None,
        }
    }
}

fn check(me: &Piece, new_position: &Position, board: &Board) -> bool {
    !super::dest_is_same_color(me, new_position, board)
}
