use crate::coord::{Coord, FILES, RANKS};
use crate::game::{Game, Side};
use crate::moves::{Move, process_candidates_in_line};
use std::collections::HashSet;

pub fn move_set(game: &Game, coord: &Coord, side: &Side) -> HashSet<Move> {
    let mut moves = HashSet::new();

    let rank_index: usize = RANKS.iter().position(|n| *n == coord.rank).unwrap();
    process_candidates_in_line(
        &game.board,
        &coord,
        RANKS[..rank_index].iter().rev().map(|rank| Coord { rank: *rank, file: coord.file }).collect(),
        &side,
        &mut moves
    );
    process_candidates_in_line(
        &game.board,
        &coord,
        RANKS[rank_index + 1..].iter().map(|rank| Coord { rank: *rank, file: coord.file }).collect(),
        &side,
        &mut moves
    );

    let file_index: usize = FILES.iter().position(|n| *n == coord.file).unwrap();
    process_candidates_in_line(
        &game.board,
        &coord,
        FILES[..file_index].iter().rev().map(|file| Coord { file: *file, rank: coord.rank }).collect(),
        &side,
        &mut moves
    );
    process_candidates_in_line(
        &game.board,
        &coord,
        FILES[file_index + 1..].iter().map(|file| Coord { file: *file, rank: coord.rank }).collect(),
        &side,
        &mut moves
    );

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rook_stands_alone() {
        let game = Game::from_fen("8/8/8/8/4R3/8/8/8 w - - 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        assert_eq!(14, set.len());
    }

    #[test]
    fn rook_blocked_by_friendly() {
        let game = Game::from_fen("8/8/8/8/4R3/8/8/4R3 w - - 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        println!("{:?}", set);
        assert_eq!(13, set.len());
    }

    #[test]
    fn rook_blocked_by_hostile() {
        let game = Game::from_fen("8/8/8/8/4R3/8/8/4r3 w - - 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        println!("{:?}", set);
        assert_eq!(14, set.len());
        assert!(set.contains(&Move::Capture(Coord::try_from("e4").unwrap(), Coord::try_from("e1").unwrap())));
    }
}
