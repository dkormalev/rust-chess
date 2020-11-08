use crate::common;
use crate::common::Color;

use crate::common::StepResultData;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::positions::ChessError::*;
use crate::ChessResult;
use num::FromPrimitive;

use crate::positions::Position;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct Board {
    contents: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Rc<RefCell<Board>> {
        let result = Rc::new(RefCell::new(Board {
            contents: Default::default(),
        }));
        Board::fill_board(&result);
        result
    }

    pub fn cell(&self, position: &Position) -> &Option<Piece> {
        &self.contents[position.vert as usize][position.hor as usize]
    }

    pub fn move_piece(
        &mut self,
        from: &Position,
        to: &Position,
        color: Color,
    ) -> ChessResult<StepResultData> {
        {
            let ref piece = self.cell(from).as_ref().ok_or_else(|| EmptyCell(*from))?;
            if piece.color != color {
                return Err(WrongColor(*from));
            }
            if !piece.movement.can_move(piece, to, self) {
                return Err(InvalidMove(*from, *to));
            }
        }
        let mut raw_result = match self.cell(to) {
            Some(piece) if color == piece.color => return Err(InvalidMove(*from, *to)),
            Some(_) => StepResultData::Eaten(self.extract_piece(to).unwrap()),
            None => StepResultData::Moved,
        };
        let mut piece = self.extract_piece(from).unwrap();
        piece.mark_as_moved();
        piece.position = *to;
        if piece.movement.is_enemy_in_check(&piece, self) {
            raw_result = StepResultData::Check(piece.color);
        }
        self.contents[to.vert as usize][to.hor as usize] = Some(piece);
        Ok(raw_result)
    }

    pub fn draw(&self) {
        print!("{}", termion::color::Fg(termion::color::Black));
        let mut color = common::Color::White;
        for i in (0u16..8).rev() {
            print!("{}", termion::cursor::Goto(3, 10 - i));
            for j in 0u16..8 {
                if color == common::Color::Black {
                    print!(
                        "{}{}",
                        termion::color::Bg(termion::color::Yellow),
                        self.piece_icon(i.into(), j.into())
                    );
                    color = common::Color::White;
                } else {
                    print!(
                        "{}{}",
                        termion::color::Bg(termion::color::White),
                        self.piece_icon(i.into(), j.into())
                    );
                    color = common::Color::Black;
                }
            }
            color = if color == common::Color::Black {
                common::Color::White
            } else {
                common::Color::Black
            }
        }
        println!(
            "{}{}",
            termion::color::Bg(termion::color::Reset),
            termion::color::Fg(termion::color::Reset)
        );
    }

    pub fn is_in_check_state(&self) -> Option<Color> {
        for i in 0..8 {
            for j in 0..8 {
                let now = Position::at(i, j)
                    .and_then(|pos| self.cell(&pos).as_ref())
                    .filter(|piece| piece.movement.is_enemy_in_check(piece, self))
                    .map(|piece| !piece.color);
                if now.is_some() {
                    return now;
                }
            }
        }
        None
    }

    fn extract_piece(&mut self, position: &Position) -> Option<Piece> {
        mem::replace(
            &mut self.contents[position.vert as usize][position.hor as usize],
            None,
        )
    }

    fn piece_icon(&self, i: usize, j: usize) -> char {
        match &self.contents[i][j] {
            Some(piece) => piece.icon(),
            None => ' ',
        }
    }

    fn fill_board(board: &Rc<RefCell<Board>>) {
        // reversed, whites on top (small letters), blacks on bottom (Capital letters)
        let board_pic: [[u8; 8]; 8] = [
            *b"rnbkqbnr",
            *b"pppppppp",
            *b"________",
            *b"________",
            *b"________",
            *b"________",
            *b"PPPPPPPP",
            *b"RNBKQBNR",
        ];

        let mut borrowed = board.borrow_mut();

        for i in 0..8 {
            for j in 0..8 {
                let piece_type = match board_pic[i][j].to_ascii_lowercase() {
                    b'r' => PieceType::Rook,
                    b'n' => PieceType::Knight,
                    b'b' => PieceType::Bishop,
                    b'k' => PieceType::King,
                    b'q' => PieceType::Queen,
                    b'p' => PieceType::Pawn,
                    _ => continue,
                };
                let color = if board_pic[i][j].is_ascii_lowercase() {
                    crate::common::Color::White
                } else {
                    crate::common::Color::Black
                };
                let position = Position {
                    hor: FromPrimitive::from_usize(j).unwrap(),
                    vert: FromPrimitive::from_usize(i).unwrap(),
                };
                borrowed.contents[i][j] = Some(Piece::new(piece_type, color, position));
            }
        }
    }
}
