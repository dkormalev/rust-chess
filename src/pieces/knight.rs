use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::PieceMovement;
use crate::pieces::StepChecker;
use crate::Position;

pub struct Knight;

impl PieceMovement for Knight {
    fn new() -> Self {
        Knight {}
    }

    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        [
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
        ]
        .iter()
        .filter_map(|(hor, vert)| piece.position.transform(*hor, *vert))
        .filter_map(|new_position| board.cell(&new_position).as_ref())
        .any(|other| super::verify_check(piece, other))
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        match diff {
            (-2, -1) => Some(Box::new(check)),
            (-2, 1) => Some(Box::new(check)),
            (2, -1) => Some(Box::new(check)),
            (2, 1) => Some(Box::new(check)),
            (-1, -2) => Some(Box::new(check)),
            (-1, 2) => Some(Box::new(check)),
            (1, -2) => Some(Box::new(check)),
            (1, 2) => Some(Box::new(check)),
            _ => None,
        }
    }
}

fn check(me: &Piece, new_position: &Position, board: &Board) -> bool {
    !super::dest_is_same_color(me, new_position, board)
}
