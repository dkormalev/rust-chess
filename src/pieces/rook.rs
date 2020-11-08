use super::parallel_movement::ParallelMovement;
use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::PieceMovement;
use crate::pieces::StepChecker;

pub struct Rook;

impl ParallelMovement for Rook {}

impl PieceMovement for Rook {
    fn new() -> Self {
        Rook {}
    }

    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        <Self as ParallelMovement>::is_enemy_in_check(self, piece, board)
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        <Self as ParallelMovement>::step_checker(self, diff)
    }
}
