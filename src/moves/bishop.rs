use crate::coord::{Coord, FILES, RANKS};
use crate::game::{Game, Side};
use crate::moves::{Move, process_candidates_in_line};
use std::collections::HashSet;

pub fn move_set(game: &Game, coord: &Coord, side: &Side) -> HashSet<Move> {
    let mut moves = HashSet::new();

    let rank_index: usize = RANKS.iter().position(|n| *n == coord.rank).unwrap();
    let file_index: usize = FILES.iter().position(|n| *n == coord.file).unwrap();
    let diag_minus_minus = std::iter::zip(
        RANKS[..rank_index].iter().rev(),
        FILES[..file_index].iter().rev(),
    )
    .map(|(rank, file)| Coord {
        rank: *rank,
        file: *file,
    })
    .collect();
    process_candidates_in_line(&game.board, &coord, diag_minus_minus, &side, &mut moves);
    let diag_minus_plus = std::iter::zip(
        RANKS[..rank_index].iter().rev(),
        FILES[file_index + 1..].iter(),
    )
    .map(|(rank, file)| Coord {
        rank: *rank,
        file: *file,
    })
    .collect();
    process_candidates_in_line(&game.board, &coord, diag_minus_plus, &side, &mut moves);
    let diag_plus_plus = std::iter::zip(
        RANKS[rank_index + 1..].iter(),
        FILES[file_index + 1..].iter(),
    )
    .map(|(rank, file)| Coord {
        rank: *rank,
        file: *file,
    })
    .collect();
    process_candidates_in_line(&game.board, &coord, diag_plus_plus, &side, &mut moves);
    let diag_plus_minus = std::iter::zip(
        RANKS[rank_index + 1..].iter(),
        FILES[..file_index].iter().rev(),
    )
    .map(|(rank, file)| Coord {
        rank: *rank,
        file: *file,
    })
    .collect();
    process_candidates_in_line(&game.board, &coord, diag_plus_minus, &side, &mut moves);

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bishop_stands_alone() {
        let game = Game::from_fen("8/8/8/8/4B3/8/8/8 w - - 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        println!("{:?}", set);
        assert_eq!(13, set.len());
    }

    #[test]
    fn bishop_blocked_by_friendly() {
        let game = Game::from_fen("R7/8/8/8/4B3/8/8/8 w - - 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        println!("{:?}", set);
        assert_eq!(12, set.len());
    }

    #[test]
    fn bishop_blocked_by_hostile() {
        let game = Game::from_fen("8/1r7/8/8/4B3/8/8/8 w - - 0 1");
        let set = Game::move_set(&game, &Coord::try_from("e4").unwrap());
        println!("{:?}", set);
        assert_eq!(12, set.len());
        assert!(set.contains(&Move::Capture(
            Coord::try_from("e4").unwrap(),
            Coord::try_from("b7").unwrap()
        )));
    }
}
