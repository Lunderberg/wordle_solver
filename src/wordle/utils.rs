use super::{Error, GameState, Word};

use std::path::Path;

impl<const N: usize> GameState<N> {
    pub fn from_files<P1: AsRef<Path>, P2: AsRef<Path>>(
        allowed_words: &P1,
        secret_words: &P2,
    ) -> Result<Self, Error> {
        let dictionary = std::fs::read_to_string(allowed_words)
            .map_err(|e| Error::WordListReadError(e))?
            .split("\n")
            .filter(|s| s.len() == N)
            .collect_words();
        let secret = std::fs::read_to_string(secret_words)
            .map_err(|e| Error::WordListReadError(e))?
            .split("\n")
            .filter(|s| s.len() == N)
            .collect_words();

        Ok(Self { dictionary, secret })
    }

    pub fn from_iter<'a>(word_iter: impl Iterator<Item = &'a str>) -> Self {
        let dictionary: Vec<Word<N>> =
            word_iter.filter(|s| s.len() == N).collect_words();
        let secret = dictionary.clone();
        Self { dictionary, secret }
    }

    fn words_from_bytes(bytes: &[u8]) -> Vec<Word<N>> {
        std::str::from_utf8(bytes)
            .unwrap()
            .split("\n")
            .collect_words()
    }

    pub fn from_scrabble() -> Self {
        let words = Self::words_from_bytes(include_bytes!("scrabble.txt"));
        Self {
            dictionary: words.clone(),
            secret: words,
        }
    }

    pub fn from_wordle() -> Self {
        let dictionary = Self::words_from_bytes(include_bytes!(
            "wordle_allowed_guesses.txt"
        ));
        let secret = Self::words_from_bytes(include_bytes!(
            "wordle_possible_secrets.txt"
        ));

        Self { dictionary, secret }
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
        self.map(|s| {
            let mut letters = ['0'; N];
            letters.iter_mut().zip(s.chars()).for_each(|(out, c)| {
                *out = c;
            });
            Word { letters }
        })
        .collect()
    }
}
