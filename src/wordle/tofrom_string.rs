use super::*;

use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl<const N: usize> Display for Word<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.letters.iter().try_for_each(|c| write!(f, "{}", c))
    }
}

impl<const N: usize> Display for Clue<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.tiles
            .iter()
            .map(|t| match t {
                Tile::Correct => 'G',
                Tile::WrongPosition => 'Y',
                Tile::NotPresentInWord => '_',
            })
            .try_for_each(|c| write!(f, "{}", c))
    }
}

impl<const N: usize> FromStr for Word<N> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if s.len() == N {
            let mut letters = ['0'; N];
            letters.iter_mut().zip(s.chars()).for_each(|(out, c)| {
                *out = c;
            });

            Ok(Self { letters })
        } else {
            Err(Error::IncorrectStringLength)
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Error> {
        match c {
            'G' => Ok(Tile::Correct),
            'Y' => Ok(Tile::WrongPosition),
            '_' => Ok(Tile::NotPresentInWord),
            _ => Err(Error::NotTileChar(c)),
        }
    }
}

impl<const N: usize> FromStr for Clue<N> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let vec_tiles: Vec<Tile> = s
            .chars()
            .map(|c| -> Result<Tile, Error> { c.try_into() })
            .collect::<Result<Vec<Tile>, Error>>()?;
        let tiles: [Tile; N] = vec_tiles
            .as_slice()
            .try_into()
            .map_err(|_| Error::IncorrectStringLength)?;
        Ok(Self { tiles })
    }
}
