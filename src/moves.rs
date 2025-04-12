use crate::board::Board;
use crate::coord::Coord;
use crate::game::Side;
use crate::piece::Piece;
use std::collections::HashSet;
use std::fmt;

pub mod pawn;
pub mod rook;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Move {
    Basic(/* origin */ Coord, /* destination */ Coord),
    Capture(/* origin */ Coord, /* destination */ Coord),
    DoubleAdvance(/* destination */ Coord, /* en passant */ Coord),
    EnPassant(/* origin */ Coord),
    Promotion(/* origin */ Coord, /* destination */ Coord, Piece),
    Castle,
}

impl Into<String> for Move {
    fn into(self) -> String {
        match self {
            Move::Basic(origin, destination) => {
                let mut debug = String::new();
                debug.push_str("Basic(origin=");
                debug.push_str(&origin.to_string());
                debug.push_str(",destination=");
                debug.push_str(&destination.to_string());
                debug.push_str(")");
                return debug;
            },
            _ => todo!()
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Basic(origin, destination) => {
                write!(f, "Basic(origin={} destination={})", &origin.to_string(), &destination.to_string())
            }
            _ => todo!()
        }
    }
}

pub fn process_candidates_in_line(
    board: &Board,
    origin: &Coord,
    candidates: Vec<Coord>,
    side: &Side,
    moves: &mut HashSet<Move>,
) {
    for candidate in candidates {
        let piece_at_candidate = Board::piece_at(&board, &candidate);
        match piece_at_candidate {
            Some(p) if Piece::side(&p) != side => {
                println!("Capture {}", candidate);
                moves.insert(Move::Capture(origin.clone(), candidate));
                return;
            }
            Some(_) => {
                return;
            }
            None => {
                println!("Basic {}", candidate);
                moves.insert(Move::Basic(origin.clone(), candidate));
            }
        }
    }
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
        if self.kingside_white {
            fen.push_str("K");
        }
        if self.queenside_white {
            fen.push_str("Q");
        }
        if self.kingside_black {
            fen.push_str("k");
        }
        if self.queenside_black {
            fen.push_str("q");
        }
        if fen.is_empty() {
            return String::from("-");
        }
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
