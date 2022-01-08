use itertools::Itertools;

#[derive(Debug)]
pub enum Error {
    IncorrectStringLength,
    NoDictionaryFile,
    DictionaryReadError(std::io::Error),
    NoWordsRemaining,
    NotTileChar(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tile {
    Correct,
    WrongPosition,
    NotPresentInWord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Word<const N: usize> {
    pub letters: [char; N],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Clue<const N: usize> {
    pub tiles: [Tile; N],
}

#[derive(Debug, Clone)]
pub struct GameState<const N: usize> {
    pub dictionary: Vec<Word<N>>,
    pub secret: Vec<Word<N>>,
}

impl<const N: usize> GameState<N> {
    pub fn new<'a>(word_iter: impl Iterator<Item = &'a String>) -> Self {
        let dictionary: Vec<Word<N>> = word_iter
            .filter(|s| s.len() == N)
            .map(|s| {
                let mut letters = ['0'; N];
                letters.iter_mut().zip(s.chars()).for_each(|(out, c)| {
                    *out = c;
                });
                Word { letters }
            })
            .collect();
        let secret = dictionary.clone();
        Self { dictionary, secret }
    }

    pub fn after_guess(
        &self,
        guess: Word<N>,
        observed_result: Clue<N>,
    ) -> Result<Self, Error> {
        let secret = self
            .secret
            .iter()
            .filter(|secret| compare_words(**secret, guess) == observed_result)
            .copied()
            .collect();
        Ok(Self {
            dictionary: self.dictionary.clone(),
            secret,
        })
    }

    pub fn simulate_strategy<F>(
        &self,
        secret_word: Word<N>,
        mut strategy: F,
    ) -> impl Iterator<Item = Result<(Option<(Word<N>, Clue<N>)>, Self), Error>>
    where
        F: FnMut(&Self) -> Word<N>,
    {
        std::iter::successors(
            Some(Ok((None, self.clone()))),
            move |res_state| {
                if let Ok((_prev_clue, state)) = res_state {
                    (state.secret.len() > 1).then(|| {
                        let guess = strategy(state);
                        let clue = compare_words(secret_word, guess);
                        state
                            .after_guess(guess, clue)
                            .map(|new_state| (Some((guess, clue)), new_state))
                    })
                } else {
                    None
                }
            },
        )
    }

    pub fn best_guess(&self) -> Result<Word<N>, Error> {
        if self.secret.len() == 0 {
            return Err(Error::NoWordsRemaining);
        }
        Ok(self
            .dictionary
            .iter()
            .min_by_key(|guess| {
                self.secret
                    .iter()
                    .map(|secret| compare_words(*secret, **guess))
                    .counts()
                    .into_values()
                    .max()
                    .unwrap()
            })
            .unwrap()
            .clone())
    }
}

pub fn compare_words<const N: usize>(
    secret_word: Word<N>,
    guess: Word<N>,
) -> Clue<N> {
    let mut tiles = [Tile::NotPresentInWord; N];

    for i in 0..N {
        tiles[i] = if guess[i] == secret_word[i] {
            Tile::Correct
        } else {
            let num_occurrences =
                secret_word.iter().filter(|c| **c == guess[i]).count();
            let i_occurrence =
                guess.iter().take(i).filter(|c| **c == guess[i]).count();
            if i_occurrence < num_occurrences {
                Tile::WrongPosition
            } else {
                Tile::NotPresentInWord
            }
        }
    }

    Clue { tiles }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("apple", "table", "_Y_GG")]
    #[case("farts", "ghost", "___YY")]
    fn test_compare(
        #[case] secret: &str,
        #[case] guess: &str,
        #[case] expected: &str,
    ) -> Result<(), Error> {
        let res = compare_words::<5>(secret.parse()?, guess.parse()?);
        let expected: Clue<5> = expected.parse()?;
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn test_after_guess() -> Result<(), Error> {
        let secret: Vec<Word<5>> = ["apple", "table", "farts", "ghost"]
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        let before = GameState {
            dictionary: secret.clone(),
            secret,
        };
        let after = before
            .after_guess("chart".parse()?, "_G__G".parse()?)
            .unwrap();

        assert_eq!(after.secret, vec!["ghost".parse()?]);
        Ok(())
    }
}
