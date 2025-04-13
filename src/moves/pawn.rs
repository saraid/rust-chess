use crate::board::Board;
use crate::coord::Coord;
use crate::game::{Game, Side};
use crate::moves::Move;
use crate::piece::Piece;
use std::collections::HashSet;

pub fn move_set(game: &Game, coord: &Coord, side: &Side) -> HashSet<Move> {
    let mut moves = HashSet::new();

    // basic
    let Some(candidate) = Coord::next_rank(&coord, &side, 1) else {
        panic!("this pawn should have been promoted");
    };
    if Board::piece_at(&game.board, &candidate).is_none() {
        moves.insert(Move::Basic(coord.clone(), candidate));
    }

    // double advance
    fn pawn_start_rank(side: &Side) -> char {
        match side {
            Side::Black => '7',
            Side::White => '2',
        }
    }

    if coord.rank == pawn_start_rank(&side) {
        let Some(candidate) = Coord::next_rank(&coord, &side, 2) else {
            panic!("this pawn isn't at start rank");
        };
        let Some(en_passant_target) = Coord::next_rank(&coord, &side, 1) else {
            panic!("this pawn isn't at start rank");
        };
        if Board::piece_at(&game.board, &en_passant_target).is_none()
            && Board::piece_at(&game.board, &candidate).is_none()
        {
            moves.insert(Move::DoubleAdvance(candidate, en_passant_target));
        }
    }

    // captures
    match Coord::next_rank(&coord, &side, 1).and_then(|c| Coord::positive_file(&c, 1)) {
        Some(candidate) => {
            match &game.en_passant_target {
                Some(_target) => {
                    moves.insert(Move::EnPassant(coord.clone()));
                }
                None => {}
            }
            match Board::piece_at(&game.board, &candidate).filter(|p| Piece::side(p) != side) {
                Some(_) => {
                    moves.insert(Move::Capture(coord.clone(), candidate));
                }
                None => {}
            }
        }
        None => {}
    }
    match Coord::next_rank(&coord, &side, 1).and_then(|c| Coord::negative_file(&c, 1)) {
        Some(candidate) => {
            match &game.en_passant_target {
                Some(_target) => {
                    moves.insert(Move::EnPassant(coord.clone()));
                }
                None => {}
            }
            match Board::piece_at(&game.board, &candidate).filter(|p| Piece::side(p) != side) {
                Some(_) => {
                    moves.insert(Move::Capture(coord.clone(), candidate));
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
        println!("e2={:?}", Board::piece_at(&game.board, &Coord::try_from("e2").unwrap()));
        let set = Game::move_set(&game, &Coord::try_from("e2").unwrap());
        println!("{:?}", set);
        assert_eq!(2, set.len());
        assert!(set.contains(&Move::Basic(
            Coord::try_from("e2").unwrap(),
            Coord::try_from("e3").unwrap()
        )));
        assert!(set.contains(&Move::DoubleAdvance(
            Coord::try_from("e4").unwrap(),
            Coord::try_from("e3").unwrap(),
        )));
    }

    #[test]
    fn pawn_moves_include_en_passant() {
        let game = Game::from_fen("8/8/8/3pP3/8/8/8/8 w KQkq d6 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e5").unwrap());
        println!("{:?}", set);
        assert_eq!(2, set.len());
        assert!(set.contains(&Move::Basic(
            Coord::try_from("e5").unwrap(),
            Coord::try_from("e6").unwrap()
        )));
        assert!(set.contains(&Move::EnPassant(Coord::try_from("e5").unwrap())));
    }

    #[test]
    fn pawn_basic_move() {
        let mut game = Game::from_fen("8/8/8/8/4P3/8/8/8 w - - 0 1");
        let piece = Board::piece_at(&game.board, &Coord::try_from("e4").unwrap()).unwrap();
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        assert_eq!(1, set.len());
        Game::execute(
            &mut game,
            Move::Basic(
                Coord::try_from("e4").unwrap(),
                Coord::try_from("e5").unwrap(),
            ),
        );
        assert_eq!(
            Board::piece_at(&game.board, &Coord::try_from("e4").unwrap()),
            None
        );
        assert_eq!(
            Board::piece_at(&game.board, &Coord::try_from("e5").unwrap()),
            Some(piece)
        );
    }
}
