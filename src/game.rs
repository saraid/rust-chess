use crate::board::Board;
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
        let board = Board::standard();

        Game {
            board,
            active_color: Side::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_player_white() {
        let game = Game::new();
        assert_eq!(Side::White, game.active_color);
    }
}
