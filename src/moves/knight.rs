use crate::coord::Coord;
use crate::game::{Game, Side};
use crate::moves::{Move, process_candidates_arbitrarily};
use std::collections::HashSet;

pub fn move_set(game: &Game, coord: &Coord, side: &Side) -> HashSet<Move> {
    let mut moves = HashSet::new();

    let candidates: Vec<Coord> = [
        (-2, -1),
        (-2, 1),
        (2, -1),
        (2, 1),
        (1, -2),
        (1, 2),
        (-1, -2),
        (-1, 2),
    ]
    .iter()
    .flat_map(|(rank_delta, file_delta)| coord.delta(*rank_delta as isize, *file_delta as isize))
    .collect();
    process_candidates_arbitrarily(&game.board, &coord, candidates, &side, &mut moves);

    moves
}
