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
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            Err(Error::InvalidString(s.to_string()))
        } else if s.len() != N {
            Err(Error::IncorrectStringLength)
        } else {
            let mut letters = [0; N];
            s.chars()
                .map(|c| c.to_ascii_uppercase())
                .map(|c| {
                    let codepoint: u32 = c.into();
                    let a: u32 = 'A'.into();
                    (codepoint - a) as u8
                })
                .zip(letters.iter_mut())
                .for_each(|(val, out)| {
                    *out = val;
                });

            Ok(Self { letters })
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
