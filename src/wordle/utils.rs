use super::{Error, GameState, Word};

use std::path::Path;

impl<const N: usize> GameState<N> {
    pub fn from_files<P1: AsRef<Path>, P2: AsRef<Path>>(
        allowed_words: &P1,
        secret_words: &P2,
    ) -> Result<Self, Error> {
        let allowed_guesses = std::fs::read_to_string(allowed_words)
            .map_err(Error::WordListReadError)?
            .split('\n')
            .filter(|s| s.len() == N)
            .collect_words();
        let possible_secrets = std::fs::read_to_string(secret_words)
            .map_err(Error::WordListReadError)?
            .split('\n')
            .filter(|s| s.len() == N)
            .collect_words();

        Ok(Self {
            allowed_guesses,
            possible_secrets,
        })
    }

    pub fn from_iter<'a>(word_iter: impl Iterator<Item = &'a str>) -> Self {
        let words: Vec<Word<N>> =
            word_iter.filter(|s| s.len() == N).collect_words();
        Self {
            allowed_guesses: words.clone(),
            possible_secrets: words,
        }
    }

    fn words_from_bytes(bytes: &[u8]) -> Vec<Word<N>> {
        std::str::from_utf8(bytes)
            .unwrap()
            .split('\n')
            .filter(|word| word.len() == N)
            .collect_words()
    }

    pub fn from_scrabble() -> Self {
        let words = Self::words_from_bytes(include_bytes!("scrabble.txt"));
        Self {
            allowed_guesses: words.clone(),
            possible_secrets: words,
        }
    }

    pub fn from_wordle() -> Self {
        let allowed_guesses = Self::words_from_bytes(include_bytes!(
            "wordle_allowed_guesses.txt"
        ));
        let possible_secrets = Self::words_from_bytes(include_bytes!(
            "wordle_possible_secrets.txt"
        ));

        Self {
            allowed_guesses,
            possible_secrets,
        }
    }
}

trait WordCollector<const N: usize> {
    fn collect_words(&mut self) -> Vec<Word<N>>;
}

impl<'a, I, const N: usize> WordCollector<N> for I
where
    I: Iterator<Item = &'a str>,
{
    fn collect_words(&mut self) -> Vec<Word<N>> {
        self.map(|s| s.parse().unwrap()).collect()
    }
}
