use crate::board::{Board, STANDARD_FEN};
//use crate::coord::Coord;
//use crate::piece::Piece;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
    Black,
    White,
}

pub struct Game {
    pub board: Board,
    pub active_color: Side,
}

impl Game {
    pub fn new() -> Self {
        let Ok(board) = Board::try_from(STANDARD_FEN) else {
            panic!("impossible fen");
        };

        Game {
            board,
            active_color: Side::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
