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
    pub fn positive_rank(&self, steps: usize) -> Option<Coord> {
        let index = RANKS.iter().position(|n| *n == self.rank);
        index.filter(|i| i + steps < RANKS.len()).map(|i| Coord {
            rank: RANKS[i + steps],
            file: self.file,
        })
    }
    pub fn negative_rank(&self, steps: usize) -> Option<Coord> {
        let index = RANKS.iter().position(|n| *n == self.rank);
        index.filter(|i| steps <= *i).map(|i| Coord {
            rank: RANKS[i - steps],
            file: self.file,
        })
    }
    pub fn next_rank(&self, side: &Side, steps: usize) -> Option<Coord> {
        match side {
            Side::Black => self.negative_rank(steps),
            Side::White => self.positive_rank(steps),
        }
    }

    pub fn positive_file(&self, steps: usize) -> Option<Coord> {
        let index = FILES.iter().position(|n| *n == self.file);
        index.filter(|i| i + steps < FILES.len()).map(|i| Coord {
            file: FILES[i + steps],
            rank: self.rank,
        })
    }
    pub fn negative_file(&self, steps: usize) -> Option<Coord> {
        let index = FILES.iter().position(|n| *n == self.file);
        index.filter(|i| steps <= *i).map(|i| Coord {
            file: FILES[i - steps],
            rank: self.rank,
        })
    }

    pub fn delta(&self, rank: isize, file: isize) -> Option<Coord> {
        if rank > 0 {
            self.positive_rank(rank as usize)
        } else {
            self.negative_rank(rank.abs() as usize)
        }
        .map(|intermediate_coord| {
            if file > 0 {
                intermediate_coord.positive_file(file as usize)
            } else {
                intermediate_coord.negative_file(file.abs() as usize)
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
        let Some(actual) = given.positive_rank(1) else {
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
        let actual = given.positive_rank(1);
        assert_eq!(None, actual);
    }

    #[test]
    fn negative_rank_normal() {
        let given = Coord {
            rank: '8',
            file: 'a',
        };
        let Some(actual) = given.negative_rank(1) else {
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
        let actual = given.negative_rank(1);
        assert_eq!(None, actual);
    }

    #[test]
    fn positive_file_normal() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let Some(actual) = given.positive_file(1) else {
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
        let actual = given.positive_file(1);
        assert_eq!(None, actual);
    }

    #[test]
    fn negative_file_normal() {
        let given = Coord {
            rank: '1',
            file: 'h',
        };
        let Some(actual) = given.negative_file(1) else {
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
        let actual = given.negative_file(1);
        assert_eq!(None, actual);
    }

    #[test]
    fn delta_normal() {
        let given = Coord {
            rank: '1',
            file: 'a',
        };
        let Some(actual) = given.delta(1, 1) else {
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
        let actual = given.delta(-1, 1);
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
