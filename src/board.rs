use crate::coord::{FILES, RANKS};
use crate::{coord::Coord, piece::Piece};
use std::collections::HashMap;

pub struct Board {
    squares: HashMap<Coord, Option<Piece>>,
}

pub const STANDARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Board {
    pub fn new() -> Board {
        let mut squares = HashMap::with_capacity(64);
        for rank in RANKS {
            for file in FILES {
                let coord = Coord { rank, file };
                squares.insert(coord, None);
            }
        }
        Board { squares }
    }

    pub fn empty() -> Board { Board::new() }

    pub fn standard() -> Board {
        let Ok(board) = Board::try_from(STANDARD_FEN) else {
            panic!("impossible fen");
        };
        board
    }

    pub fn piece_at(board: &Self, coord: &Coord) -> Option<Piece> {
        match board.squares.get(&coord) {
            Some(piece_opt) => *piece_opt,
            _ => panic!(),
        }
    }

    pub fn place(board: &mut Self, coord: &Coord, piece: Piece) {
        board
            .squares
            .entry(coord.clone())
            .and_modify(|s| *s = Some(piece));
    }

    pub fn remove(board: &mut Self, coord: &Coord) {
        board
            .squares
            .entry(coord.clone())
            .and_modify(|s| *s = None);
    }
}

impl TryFrom<&str> for Board {
    type Error = ();

    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let mut board = Board::new();
        let mut rank_index: usize = 0;
        for fen_rank in fen.split("/") {
            let mut file_index: usize = 0;
            for rank_part in fen_rank.chars() {
                match rank_part {
                    '1'..='8' => {
                        //println!("Processing {}", rank_part);
                        file_index += rank_part.to_digit(10).map(|x| x as usize).unwrap();
                    }
                    'r' | 'R' | 'n' | 'N' | 'b' | 'B' | 'q' | 'Q' | 'k' | 'K' | 'p' | 'P' => {
                        //println!("Processing {}", rank_part);
                        let coord = Coord {
                            rank: RANKS[rank_index],
                            file: FILES[file_index],
                        };
                        let piece = Piece::try_from(rank_part)?;
                        Board::place(&mut board, &coord, piece);
                        file_index += 1;
                    }
                    _ => {
                        //println!("Buh? {}", rank_part);
                        return Err(());
                    }
                }
            }
            rank_index += 1;
        }
        Ok(board)
    }
}

impl Into<String> for Board {
    fn into(self) -> String {
        let mut fen_parts = Vec::<String>::with_capacity(8);
        for rank in RANKS {
            let mut rank_fen = String::new();
            let mut empty_count = 0;
            for file in FILES {
                let piece_option = Self::piece_at(&self, &Coord { rank, file });
                match &piece_option {
                    Some(piece) => {
                        if empty_count > 0 {
                            rank_fen.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        rank_fen.push_str((*piece).into());
                    }
                    None => empty_count += 1,
                }
            }
            if empty_count > 0 {
                rank_fen.push_str(&empty_count.to_string());
            }
            fen_parts.push(rank_fen);
        }
        fen_parts.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Side;

    #[test]
    fn standard_fen_try_from_works() {
        let Ok(_) = Board::try_from(STANDARD_FEN) else {
            panic!("impossible fen");
        };
    }

    #[test]
    fn there_and_back_again() {
        let Ok(board) = Board::try_from(STANDARD_FEN) else {
            panic!("impossible fen");
        };
        assert_eq!(String::from(STANDARD_FEN), Into::<String>::into(board));
    }

    #[test]
    fn place_normal() {
        let coord = Coord { rank: 'a', file: '1' };
        let piece = Piece::Rook(Side::White);
        let mut board = Board::empty();
        Board::place(&mut board, &coord, piece);
        assert_eq!(Some(piece), Board::piece_at(&board, &coord));
    }

    #[test]
    fn place_overwrites() {
        let coord = Coord { rank: 'a', file: '1' };
        let piece = Piece::Pawn(Side::Black);
        let mut board = Board::standard();
        Board::place(&mut board, &coord, piece);
        assert_eq!(Some(piece), Board::piece_at(&board, &coord));
    }

    #[test]
    fn remove_normal() {
        let mut board = Board::standard();
        let coord = Coord { rank: 'a', file: '1' };
        Board::remove(&mut board, &coord);
        assert_eq!(None, Board::piece_at(&board, &coord));
    }

    #[test]
    fn remove_empty() {
        let mut board = Board::standard();
        let coord = Coord { rank: 'e', file: '1' };
        Board::remove(&mut board, &coord);
        assert_eq!(None, Board::piece_at(&board, &coord));
    }
}
