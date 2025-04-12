use crate::board::Board;
use crate::coord::Coord;
use crate::piece::Piece;
use std::collections::HashSet;

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

    pub fn move_set(game: &Self, coord: &Coord) -> HashSet<Coord> {
        match Board::piece_at(&game.board, &coord) {
            Some(piece) => {
                let mut moves = HashSet::new();
                match piece {
                    Piece::Pawn(side) => {
                        // basic
                        let Some(candidate) = Coord::next_rank(&coord, &side, 1) else {
                            panic!("this pawn should have been promoted");
                        };
                        if Board::piece_at(&game.board, &candidate).is_none() {
                            moves.insert(candidate);
                        }

                        // double advance
                        fn pawn_start_rank(side: &Side) -> char {
                            match side {
                                Side::White => 'b',
                                Side::Black => 'g',
                            }
                        }

                        if coord.rank == pawn_start_rank(&side) {
                            let Some(candidate) = Coord::next_rank(&coord, &side, 2) else {
                                panic!("this pawn should have been promoted");
                            };
                            if Board::piece_at(&game.board, &candidate).is_none() {
                                moves.insert(candidate);
                            }
                        }

                        // captures
                        match Coord::next_rank(&coord, &side, 1)
                            .and_then(|c| Coord::positive_file(&c, 1))
                        {
                            Some(candidate) => {
                                match Board::piece_at(&game.board, &candidate)
                                    .filter(|p| Piece::side(p) != &side)
                                {
                                    Some(_) => {
                                        moves.insert(candidate);
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                        match Coord::next_rank(&coord, &side, 1)
                            .and_then(|c| Coord::negative_file(&c, 1))
                        {
                            Some(candidate) => {
                                match Board::piece_at(&game.board, &candidate)
                                    .filter(|p| Piece::side(p) != &side)
                                {
                                    Some(_) => {
                                        moves.insert(candidate);
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }

                        // en passant TODO
                    }
                    _ => panic!(),
                }
                moves
            }
            None => HashSet::new(),
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

    #[test]
    fn pawn_moves_from_start() {
        let game = Game::new();
        let set = Game::move_set(&game, &Coord { rank: 'e', file: '2' });
        println!("{:?}", set);
        assert!(set.contains(&Coord { rank: 'f', file: '2' }));
    }
}
