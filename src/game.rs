use crate::board::Board;
use crate::coord::Coord;
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

pub struct CastlingAvailability {
    kingside_white: bool,
    queenside_white: bool,
    kingside_black: bool,
    queenside_black: bool,
}

pub enum Castle {
    Kingside(Side),
    Queenside(Side),
}

impl CastlingAvailability {
    pub fn all() -> Self {
        Self {
            kingside_white: true,
            queenside_white: true,
            kingside_black: true,
            queenside_black: true,
        }
    }

    pub fn none() -> Self {
        Self {
            kingside_white: false,
            queenside_white: false,
            kingside_black: false,
            queenside_black: false,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        if fen == "-" {
            return Self::none();
        }

        let mut ca = Self::none();
        for part in fen.chars() {
            match part {
                'K' => ca.kingside_white = true,
                'Q' => ca.queenside_white = true,
                'k' => ca.kingside_black = true,
                'q' => ca.queenside_black = true,
                _ => panic!("invalid fen value"),
            }
        }
        ca
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        if self.kingside_white { fen.push_str("K"); }
        if self.queenside_white { fen.push_str("Q"); }
        if self.kingside_black { fen.push_str("k"); }
        if self.queenside_black { fen.push_str("q"); }
        if fen.is_empty()
        { return String::from("-"); }
        fen
    }

    pub fn available(&self, castle: &Castle) -> bool {
        match castle {
            Castle::Kingside(side) => match side {
                Side::White => self.kingside_white,
                Side::Black => self.kingside_black,
            },
            Castle::Queenside(side) => match side {
                Side::White => self.queenside_white,
                Side::Black => self.queenside_black,
            },
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Move {
    Basic(/* origin */ Coord, /* destination */ Coord),
    Capture(/* origin */ Coord, /* destination */ Coord),
    DoubleAdvance(/* destination */ Coord, /* en passant */ Coord),
    EnPassant(/* origin */ Coord),
    Promotion(/* origin */ Coord, /* destination */ Coord, Piece),
    Castle,
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
            Some(piece) => {
                let mut moves = HashSet::new();
                match piece {
                    Piece::Pawn(side) => {
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
                        match Coord::next_rank(&coord, &side, 1)
                            .and_then(|c| Coord::positive_file(&c, 1))
                        {
                            Some(candidate) => {
                                match &game.en_passant_target {
                                    Some(_target) => {
                                        moves.insert(Move::EnPassant(coord.clone()));
                                    }
                                    None => {}
                                }
                                match Board::piece_at(&game.board, &candidate)
                                    .filter(|p| Piece::side(p) != &side)
                                {
                                    Some(_) => {
                                        moves.insert(Move::Capture(coord.clone(), candidate));
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
                                match &game.en_passant_target {
                                    Some(_target) => {
                                        moves.insert(Move::EnPassant(coord.clone()));
                                    }
                                    None => {}
                                }
                                match Board::piece_at(&game.board, &candidate)
                                    .filter(|p| Piece::side(p) != &side)
                                {
                                    Some(_) => {
                                        moves.insert(Move::Capture(coord.clone(), candidate));
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                    }
                    _ => todo!(),
                }
                moves
            }
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

    #[test]
    fn pawn_moves_from_start() {
        let game = Game::new();
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
        assert!(set.contains(&Move::Capture(Coord::try_from("d6").unwrap())));
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
