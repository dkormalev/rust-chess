use super::diagonal_movement::DiagonalMovement;
use super::parallel_movement::ParallelMovement;
use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::PieceMovement;
use crate::pieces::StepChecker;

pub struct Queen;

impl DiagonalMovement for Queen {}
impl ParallelMovement for Queen {}

impl PieceMovement for Queen {
    fn new() -> Self {
        Queen {}
    }

    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        <Self as DiagonalMovement>::is_enemy_in_check(self, piece, board)
            || <Self as ParallelMovement>::is_enemy_in_check(self, piece, board)
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        <Self as DiagonalMovement>::step_checker(self, diff)
            .or_else(|| <Self as ParallelMovement>::step_checker(self, diff))
    }
}
