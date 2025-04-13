use crate::game::Side;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Piece {
    King(Side),
    Queen(Side),
    Bishop(Side),
    Knight(Side),
    Rook(Side),
    Pawn(Side),
}

impl Piece {
    pub fn side(&self) -> &Side {
        match self {
            Piece::King(side) => side,
            Piece::Queen(side) => side,
            Piece::Bishop(side) => side,
            Piece::Knight(side) => side,
            Piece::Rook(side) => side,
            Piece::Pawn(side) => side,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();
    fn try_from(fen: char) -> Result<Self, Self::Error> {
        match fen {
            'r' => Ok(Piece::Rook(Side::Black)),
            'n' => Ok(Piece::Knight(Side::Black)),
            'b' => Ok(Piece::Bishop(Side::Black)),
            'q' => Ok(Piece::Queen(Side::Black)),
            'k' => Ok(Piece::King(Side::Black)),
            'p' => Ok(Piece::Pawn(Side::Black)),
            'R' => Ok(Piece::Rook(Side::White)),
            'N' => Ok(Piece::Knight(Side::White)),
            'B' => Ok(Piece::Bishop(Side::White)),
            'Q' => Ok(Piece::Queen(Side::White)),
            'K' => Ok(Piece::King(Side::White)),
            'P' => Ok(Piece::Pawn(Side::White)),
            _ => Err(()),
        }
    }
}

impl Into<&str> for Piece {
    fn into(self) -> &'static str {
        match self {
            Piece::King(Side::Black) => "k",
            Piece::Queen(Side::Black) => "q",
            Piece::Bishop(Side::Black) => "b",
            Piece::Knight(Side::Black) => "n",
            Piece::Rook(Side::Black) => "r",
            Piece::Pawn(Side::Black) => "p",
            Piece::King(Side::White) => "K",
            Piece::Queen(Side::White) => "Q",
            Piece::Bishop(Side::White) => "B",
            Piece::Knight(Side::White) => "N",
            Piece::Rook(Side::White) => "R",
            Piece::Pawn(Side::White) => "P",
        }
    }
}
