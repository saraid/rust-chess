pub enum Side {
    Black,
    White,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    pub rank: char,
    pub file: char,
}

pub const RANKS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const FILES: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

impl Coord {
    pub fn positive_rank(coord: &Self, steps: usize) -> Option<Coord> {
        let index = RANKS.iter().position(|n| *n == coord.rank);
        index.filter(|i| i + steps < RANKS.len()).map(|i| Coord {
            rank: RANKS[i + steps],
            file: coord.file,
        })
    }
    pub fn negative_rank(coord: &Self, steps: usize) -> Option<Coord> {
        let index = RANKS.iter().position(|n| *n == coord.rank);
        index.filter(|i| steps <= *i).map(|i| Coord {
            rank: RANKS[i - steps],
            file: coord.file,
        })
    }
    pub fn next_rank(coord: &Self, side: &Side, steps: usize) -> Option<Coord> {
        match side {
            Side::Black => Self::negative_rank(&coord, steps),
            Side::White => Self::positive_rank(&coord, steps),
        }
    }

    pub fn positive_file(coord: &Self, steps: usize) -> Option<Coord> {
        let index = FILES.iter().position(|n| *n == coord.file);
        index.filter(|i| i + steps < FILES.len()).map(|i| Coord {
            file: FILES[i + steps],
            rank: coord.rank,
        })
    }
    pub fn negative_file(coord: &Self, steps: usize) -> Option<Coord> {
        let index = FILES.iter().position(|n| *n == coord.file);
        index.filter(|i| steps <= *i).map(|i| Coord {
            file: FILES[i - steps],
            rank: coord.rank,
        })
    }
    /*
    pub fn next_file(coord: &Self, side: &Side, steps: usize) -> Option<Coord> {
        match side {
            Side::Black => Self::negative_file(&coord, steps),
            Side::White => Self::positive_file(&coord, steps),
        }
    }
    */

    pub fn delta(coord: &Self, rank: isize, file: isize) -> Option<Coord> {
        if rank > 0 {
            Self::positive_rank(coord, rank as usize)
        } else {
            Self::negative_rank(coord, rank.abs() as usize)
        }
        .map(|intermediate_coord| {
            if file > 0 {
                Self::positive_file(&intermediate_coord, file as usize)
            } else {
                Self::negative_file(&intermediate_coord, file.abs() as usize)
            }
        })?
    }
}

#[derive(Debug, PartialEq)]
pub enum CoordParseError {
    BadFile,
    BadRank,
}

impl TryFrom<&str> for Coord {
    type Error = CoordParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut chars = string.chars();
        let Some(rank) = chars.next().filter(|r| RANKS.contains(r)) else {
            return Err(CoordParseError::BadRank);
        };
        let Some(file) = chars.next().filter(|f| FILES.contains(f)) else {
            return Err(CoordParseError::BadFile);
        };
        Ok(Coord { rank, file })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_rank_normal() {
        let given = Coord {
            rank: 'a',
            file: '1',
        };
        let Some(actual) = Coord::positive_rank(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: 'b',
                file: '1'
            },
            actual
        )
    }

    #[test]
    fn positive_rank_invalid() {
        let given = Coord {
            rank: 'h',
            file: '1',
        };
        let actual = Coord::positive_rank(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn negative_rank_normal() {
        let given = Coord {
            rank: 'h',
            file: '1',
        };
        let Some(actual) = Coord::negative_rank(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: 'g',
                file: '1'
            },
            actual
        )
    }

    #[test]
    fn negative_rank_invalid() {
        let given = Coord {
            rank: 'a',
            file: '1',
        };
        let actual = Coord::negative_rank(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn positive_file_normal() {
        let given = Coord {
            rank: 'a',
            file: '1',
        };
        let Some(actual) = Coord::positive_file(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: 'a',
                file: '2'
            },
            actual
        )
    }

    #[test]
    fn positive_file_invalid() {
        let given = Coord {
            rank: 'a',
            file: '8',
        };
        let actual = Coord::positive_file(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn negative_file_normal() {
        let given = Coord {
            rank: 'a',
            file: '2',
        };
        let Some(actual) = Coord::negative_file(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: 'a',
                file: '1'
            },
            actual
        )
    }

    #[test]
    fn negative_file_invalid() {
        let given = Coord {
            rank: 'a',
            file: '1',
        };
        let actual = Coord::negative_file(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn delta_normal() {
        let given = Coord {
            rank: 'a',
            file: '1',
        };
        let Some(actual) = Coord::delta(&given, 1, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: 'b',
                file: '2'
            },
            actual
        );
    }

    #[test]
    fn delta_invalid() {
        let given = Coord {
            rank: 'a',
            file: '1',
        };
        let actual = Coord::delta(&given, -1, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn from_valid() {
        let Ok(coord) = Coord::try_from("a1") else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: 'a',
                file: '1'
            },
            coord
        );
    }

    #[test]
    fn from_invalid() {
        let Err(error) = Coord::try_from("a9") else {
            todo!()
        };
        assert_eq!(CoordParseError::BadFile, error);
    }
}
