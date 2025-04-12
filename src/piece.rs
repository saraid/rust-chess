use crate::game::Side;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Piece {
    King(Side),
    Queen(Side),
    Bishop(Side),
    Knight(Side),
    Rook(Side),
    Pawn(Side),
}

impl TryFrom<char> for Piece {
    type Error = ();
    fn try_from(fen: char) -> Result<Self, Self::Error> {
        println!("Piece#try_from {}", fen);
        match fen {
            'r' => Ok(Piece::Rook(Side::White)),
            'n' => Ok(Piece::Knight(Side::White)),
            'b' => Ok(Piece::Bishop(Side::White)),
            'q' => Ok(Piece::Queen(Side::White)),
            'k' => Ok(Piece::King(Side::White)),
            'p' => Ok(Piece::Pawn(Side::White)),
            'R' => Ok(Piece::Rook(Side::Black)),
            'N' => Ok(Piece::Knight(Side::Black)),
            'B' => Ok(Piece::Bishop(Side::Black)),
            'Q' => Ok(Piece::Queen(Side::Black)),
            'K' => Ok(Piece::King(Side::Black)),
            'P' => Ok(Piece::Pawn(Side::Black)),
            _ => Err(()),
        }
    }
}

impl Into<&str> for Piece {
    fn into(self) -> &'static str {
        match self {
            Piece::King(Side::White) => "k",
            Piece::Queen(Side::White) => "q",
            Piece::Bishop(Side::White) => "b",
            Piece::Knight(Side::White) => "n",
            Piece::Rook(Side::White) => "r",
            Piece::Pawn(Side::White) => "p",
            Piece::King(Side::Black) => "K",
            Piece::Queen(Side::Black) => "Q",
            Piece::Bishop(Side::Black) => "B",
            Piece::Knight(Side::Black) => "N",
            Piece::Rook(Side::Black) => "R",
            Piece::Pawn(Side::Black) => "P",
        }
    }
}
