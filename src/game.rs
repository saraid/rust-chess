use crate::board::Board;
use crate::coord::Coord;
use crate::moves::{CastlingAvailability, Move, pawn};
use crate::piece::Piece;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Side {
    Black,
    White,
}

pub struct Game {
    pub board: Board,
    pub active_color: Side,
    pub castling_availability: CastlingAvailability,
    pub en_passant_target: Option<Coord>,
    pub half_move_clock: u32,
    pub full_move_clock: u32,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::standard();

        Game {
            board,
            active_color: Side::White,
            castling_availability: CastlingAvailability::all(),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_clock: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Game {
        let game_fen_regex = Regex::new(r"(?<placement>[A-Za-z1-8/]+) (?<active_color>[wb]) (?<castling_availability>[KQkq]{1,4}|-) (?<en_passant_target>(?:[a-h][1-8])|-) (?<half_move_clock>\d+) (?<full_move_clock>\d+)").unwrap();
        let Some(caps) = game_fen_regex.captures(fen) else {
            panic!("not a game fen");
        };
        Game {
            board: Board::try_from(&caps["placement"]).unwrap(),
            active_color: match &caps["active_color"] {
                "w" => Side::White,
                "b" => Side::Black,
                _ => panic!("invalid active color"),
            },
            castling_availability: CastlingAvailability::from_fen(&caps["castling_availability"]),
            en_passant_target: Coord::try_from(&caps["en_passant_target"]).ok(),
            half_move_clock: (&caps["half_move_clock"]).parse::<u32>().unwrap(),
            full_move_clock: (&caps["full_move_clock"]).parse::<u32>().unwrap(),
        }
    }

    pub fn to_fen(game: &Self) -> String {
        let mut fen = Into::<String>::into(game.board.clone());
        fen.push_str(" ");
        match game.active_color {
            Side::White => fen.push_str("w"),
            Side::Black => fen.push_str("b"),
        }
        fen.push_str(" ");
        fen.push_str(&CastlingAvailability::to_fen(&game.castling_availability));
        fen.push_str(" ");
        match &game.en_passant_target {
            Some(target) => fen.push_str(&target.to_string()),
            None => fen.push_str("-"),
        }
        fen.push_str(" ");
        fen.push_str(&game.half_move_clock.to_string());
        fen.push_str(" ");
        fen.push_str(&game.full_move_clock.to_string());
        fen
    }

    pub fn move_set(game: &Self, coord: &Coord) -> HashSet<Move> {
        match Board::piece_at(&game.board, &coord) {
            Some(piece) => match piece {
                Piece::Pawn(side) => pawn::move_set(&game, &coord, &side),
                _ => todo!(),
            },
            None => HashSet::new(),
        }
    }

    pub fn execute(game: &mut Self, move_to_take: Move) {
        match move_to_take {
            Move::Basic(origin, destination) => {
                let piece = Board::piece_at(&game.board, &origin).unwrap();
                Board::remove(&mut game.board, &origin);
                Board::place(&mut game.board, &destination, piece);
            }
            _ => todo!(),
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
