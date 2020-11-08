use crate::pieces::Board;
use crate::pieces::Piece;
use crate::pieces::StepChecker;
use crate::positions::*;
use num::FromPrimitive;

pub trait ParallelMovement {
    fn is_enemy_in_check(&self, piece: &Piece, board: &Board) -> bool {
        [
            horizontal_path(piece.position.hor, Horizontal::HorA, piece.position.vert),
            horizontal_path(piece.position.hor, Horizontal::HorH, piece.position.vert),
            vertical_path(piece.position.vert, Vertical::Vert1, piece.position.hor),
            vertical_path(piece.position.vert, Vertical::Vert8, piece.position.hor),
        ]
        .iter()
        .any(|path| super::find_check_state_in_path(path, piece, board))
    }

    fn step_checker(&self, diff: (i8, i8)) -> Option<StepChecker> {
        match diff {
            (0, x) if x != 0 => Some(Box::new(check_vertical)),
            (x, 0) if x != 0 => Some(Box::new(check_horizontal)),
            _ => None,
        }
    }
}

fn check_horizontal(me: &Piece, new_position: &Position, board: &Board) -> bool {
    let path = horizontal_path(me.position.hor, new_position.hor, new_position.vert);
    !super::dest_is_same_color(me, new_position, board) && super::path_is_empty(&path, board)
}

fn check_vertical(me: &Piece, new_position: &Position, board: &Board) -> bool {
    let path = vertical_path(me.position.vert, new_position.vert, new_position.hor);
    !super::dest_is_same_color(me, new_position, board) && super::path_is_empty(&path, board)
}

fn horizontal_path(from: Horizontal, to: Horizontal, vert: Vertical) -> Vec<Position> {
    let (from_u8, to_u8) = if from > to {
        (to as u8, from as u8)
    } else {
        (from as u8, to as u8)
    };
    let mut result = Vec::<Position>::new();
    for i in from_u8..=to_u8 {
        let hor = FromPrimitive::from_u8(i).unwrap();
        result.push(Position { hor, vert });
    }
    if from > to {
        result.reverse();
    }
    result
}

fn vertical_path(from: Vertical, to: Vertical, hor: Horizontal) -> Vec<Position> {
    let (from_u8, to_u8) = if from > to {
        (to as u8, from as u8)
    } else {
        (from as u8, to as u8)
    };
    let mut result = Vec::<Position>::new();
    for i in from_u8..=to_u8 {
        let vert = FromPrimitive::from_u8(i).unwrap();
        result.push(Position { hor, vert });
    }
    if from > to {
        result.reverse();
    }
    result
}
