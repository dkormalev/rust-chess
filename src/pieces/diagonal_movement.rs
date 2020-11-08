use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::StepChecker;
use crate::positions::*;
use num::FromPrimitive;

pub trait DiagonalMovement {
    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        [
            uphill_path(&piece.position, Horizontal::HorA),
            uphill_path(&piece.position, Horizontal::HorH),
            downhill_path(&piece.position, Horizontal::HorA),
            downhill_path(&piece.position, Horizontal::HorH),
        ]
        .iter()
        .any(|path| super::find_check_state_in_path(path, piece, board))
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        match diff {
            (i, j) if i == j && i != 0 => Some(Box::new(check_uphill)),
            (i, j) if i == -j && i != 0 => Some(Box::new(check_downhill)),
            _ => None,
        }
    }
}

fn check_uphill(me: &Piece, new_position: &Position, board: &Board) -> bool {
    let path = uphill_path(&me.position, new_position.hor);
    !super::dest_is_same_color(me, new_position, board) && super::path_is_empty(&path, board)
}

fn check_downhill(me: &Piece, new_position: &Position, board: &Board) -> bool {
    let path = downhill_path(&me.position, new_position.hor);
    !super::dest_is_same_color(me, new_position, board) && super::path_is_empty(&path, board)
}

fn uphill_path(from: &Position, to: Horizontal) -> Vec<Position> {
    let (from_i8, to_i8, start_vert_i8) = if from.hor > to {
        (
            to as i8,
            from.hor as i8,
            from.vert as i8 - (from.hor as i8 - to as i8),
        )
    } else {
        (from.hor as i8, to as i8, from.vert as i8)
    };
    let mut result = Vec::<Position>::new();
    let mut vert_i8 = start_vert_i8;
    for i in from_i8..=to_i8 {
        let current_vert_i8 = vert_i8;
        vert_i8 = vert_i8 + 1;
        let hor = match FromPrimitive::from_i8(i) {
            Some(x) => x,
            _ => continue,
        };
        let vert = match FromPrimitive::from_i8(current_vert_i8) {
            Some(x) => x,
            _ => continue,
        };
        result.push(Position { hor, vert });
    }
    if from.hor > to {
        result.reverse();
    }
    result
}

fn downhill_path(from: &Position, to: Horizontal) -> Vec<Position> {
    let (from_i8, to_i8, start_vert_i8) = if from.hor > to {
        (
            to as i8,
            from.hor as i8,
            from.vert as i8 + (from.hor as i8 - to as i8),
        )
    } else {
        (from.hor as i8, to as i8, from.vert as i8)
    };
    let mut result = Vec::<Position>::new();
    let mut vert_i8 = start_vert_i8;
    for i in from_i8..=to_i8 {
        let current_vert_i8 = vert_i8;
        vert_i8 = vert_i8 - 1;
        let hor = match FromPrimitive::from_i8(i) {
            Some(x) => x,
            _ => continue,
        };
        let vert = match FromPrimitive::from_i8(current_vert_i8) {
            Some(x) => x,
            _ => continue,
        };
        result.push(Position { hor, vert });
    }
    if from.hor > to {
        result.reverse();
    }
    result
}
