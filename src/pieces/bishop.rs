use super::diagonal_movement::DiagonalMovement;
use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::PieceMovement;
use crate::pieces::StepChecker;

pub struct Bishop;

impl DiagonalMovement for Bishop {}

impl PieceMovement for Bishop {
    fn new() -> Self {
        Bishop {}
    }

    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        <Self as DiagonalMovement>::is_enemy_in_check(self, piece, board)
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        <Self as DiagonalMovement>::step_checker(self, diff)
    }
}
