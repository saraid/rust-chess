use crate::game::Side;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    pub rank: char,
    pub file: char,
}

pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

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
            Side::Black => Self::positive_rank(&coord, steps),
            Side::White => Self::negative_rank(&coord, steps),
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
        let Some(file) = chars.next().filter(|f| FILES.contains(f)) else {
            return Err(CoordParseError::BadFile);
        };
        let Some(rank) = chars.next().filter(|r| RANKS.contains(r)) else {
            return Err(CoordParseError::BadRank);
        };
        Ok(Coord { rank, file })
    }
}

impl Into<String> for Coord {
    fn into(self) -> String {
        let mut san = String::new();
        san.push_str(&self.file.to_string());
        san.push_str(&self.rank.to_string());
        san
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", &self.file.to_string(), &self.rank.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_rank_normal() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let Some(actual) = Coord::positive_rank(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: '2',
                file: 'a'
            },
            actual
        )
    }

    #[test]
    fn positive_rank_invalid() {
        let given = Coord {
            rank: '8',
            file: 'a',
        };
        let actual = Coord::positive_rank(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn negative_rank_normal() {
        let given = Coord {
            rank: '8',
            file: 'a',
        };
        let Some(actual) = Coord::negative_rank(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: '7',
                file: 'a'
            },
            actual
        )
    }

    #[test]
    fn negative_rank_invalid() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let actual = Coord::negative_rank(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn positive_file_normal() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let Some(actual) = Coord::positive_file(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: '1',
                file: 'b'
            },
            actual
        )
    }

    #[test]
    fn positive_file_invalid() {
        let given = Coord {
            rank: '1',
            file: 'h',
        };
        let actual = Coord::positive_file(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn negative_file_normal() {
        let given = Coord {
            rank: '1',
            file: 'h',
        };
        let Some(actual) = Coord::negative_file(&given, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: '1',
                file: 'g'
            },
            actual
        )
    }

    #[test]
    fn negative_file_invalid() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let actual = Coord::negative_file(&given, 1);
        assert_eq!(None, actual);
    }

    #[test]
    fn delta_normal() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let Some(actual) = Coord::delta(&given, 1, 1) else {
            todo!()
        };
        assert_eq!(
            Coord {
                rank: '2',
                file: 'b'
            },
            actual
        );
    }

    #[test]
    fn delta_invalid() {
        let given = Coord {
            rank: '1',
            file: 'a',
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
                rank: '1',
                file: 'a'
            },
            coord
        );
    }

    #[test]
    fn from_invalid_rank() {
        assert_eq!(Coord::try_from("a9").err(), Some(CoordParseError::BadRank));
    }

    #[test]
    fn from_invalid_file() {
        assert_eq!(Coord::try_from("z1").err(), Some(CoordParseError::BadFile));
    }
}
