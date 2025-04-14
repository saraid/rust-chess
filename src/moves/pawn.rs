use crate::coord::Coord;
use crate::game::{Game, Side};
use crate::moves::Move;
use std::collections::HashSet;

pub fn move_set(game: &Game, coord: &Coord, side: &Side) -> HashSet<Move> {
    let mut moves = HashSet::new();

    // basic
    let Some(candidate) = coord.next_rank(&side, 1) else {
        panic!("this pawn should have been promoted");
    };
    if game.board.piece_at(&candidate).is_none() {
        moves.insert(Move::Basic {
            origin: coord.clone(),
            destination: candidate,
        });
    }

    // double advance
    fn pawn_start_rank(side: &Side) -> char {
        match side {
            Side::Black => '7',
            Side::White => '2',
        }
    }

    if coord.rank == pawn_start_rank(&side) {
        let Some(candidate) = coord.next_rank(&side, 2) else {
            panic!("this pawn isn't at start rank");
        };
        let Some(en_passant_target) = coord.next_rank(&side, 1) else {
            panic!("this pawn isn't at start rank");
        };
        if game.board.piece_at(&en_passant_target).is_none()
            && game.board.piece_at(&candidate).is_none()
        {
            moves.insert(Move::DoubleAdvance {
                destination: candidate,
                en_passant_target,
            });
        }
    }

    // captures
    match coord.next_rank(&side, 1).and_then(|c| c.positive_file(1)) {
        Some(candidate) => {
            match &game.en_passant_target {
                Some(_target) => {
                    moves.insert(Move::EnPassant {
                        origin: coord.clone(),
                    });
                }
                None => {}
            }
            match game.board.piece_at(&candidate).filter(|p| p.side() != side) {
                Some(_) => {
                    moves.insert(Move::Capture {
                        origin: coord.clone(),
                        destination: candidate,
                    });
                }
                None => {}
            }
        }
        None => {}
    }
    match coord.next_rank(&side, 1).and_then(|c| c.negative_file(1)) {
        Some(candidate) => {
            match &game.en_passant_target {
                Some(_target) => {
                    moves.insert(Move::EnPassant {
                        origin: coord.clone(),
                    });
                }
                None => {}
            }
            match game.board.piece_at(&candidate).filter(|p| p.side() != side) {
                Some(_) => {
                    moves.insert(Move::Capture {
                        origin: coord.clone(),
                        destination: candidate,
                    });
                }
                None => {}
            }
        }
        None => {}
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_moves_from_start() {
        let game = Game::new();
        let set = Game::move_set(&game, &Coord::try_from("e2").unwrap());
        println!("{:?}", set);
        assert_eq!(2, set.len());
        assert!(set.contains(&Move::Basic {
            origin: Coord::try_from("e2").unwrap(),
            destination: Coord::try_from("e3").unwrap()
        }));
        assert!(set.contains(&Move::DoubleAdvance {
            destination: Coord::try_from("e4").unwrap(),
            en_passant_target: Coord::try_from("e3").unwrap(),
        }));
    }

    #[test]
    fn pawn_moves_include_en_passant() {
        let game = Game::from_fen("8/8/8/3pP3/8/8/8/8 w KQkq d6 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e5").unwrap());
        println!("{:?}", set);
        assert_eq!(2, set.len());
        assert!(set.contains(&Move::Basic {
            origin: Coord::try_from("e5").unwrap(),
            destination: Coord::try_from("e6").unwrap()
        }));
        assert!(set.contains(&Move::EnPassant {
            origin: Coord::try_from("e5").unwrap()
        }));
    }
}
