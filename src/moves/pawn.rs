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
            Side::White => '7',
            Side::Black => '2',
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
