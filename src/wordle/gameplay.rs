use super::Strategy;

#[derive(Debug)]
pub enum Error {
    IncorrectStringLength,
    NoDictionaryFile,
    WordListReadError(std::io::Error),
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
    pub allowed_guesses: Vec<Word<N>>,
    pub possible_secrets: Vec<Word<N>>,
}

impl<const N: usize> Word<N> {
    pub fn compare_with_guess(&self, guess: Word<N>) -> Clue<N> {
        let mut tiles = [Tile::NotPresentInWord; N];

        for i in 0..N {
            tiles[i] = if guess[i] == self[i] {
                Tile::Correct
            } else {
                let num_occurrences =
                    self.iter().filter(|c| **c == guess[i]).count();
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
}

impl<const N: usize> GameState<N> {
    pub fn after_guess(
        &self,
        guess: Word<N>,
        observed_result: Clue<N>,
    ) -> Result<Self, Error> {
        let secret = self
            .possible_secrets
            .iter()
            .filter(|secret| {
                secret.compare_with_guess(guess) == observed_result
            })
            .copied()
            .collect();
        Ok(Self {
            allowed_guesses: self.allowed_guesses.clone(),
            possible_secrets: secret,
        })
    }

    pub fn simulate_strategy<'a, S: Strategy<N>>(
        &self,
        secret_word: Word<N>,
        strategy: &'a mut S,
    ) -> impl Iterator<Item = Result<(Option<(Word<N>, Clue<N>)>, Self), Error>> + 'a
    {
        std::iter::successors(
            Some(Ok((None, self.clone()))),
            move |res_state| {
                if let Ok((_prev_clue, state)) = res_state {
                    (state.possible_secrets.len() > 1).then(|| {
                        let guess = strategy.make_guess(state)?;
                        let clue = secret_word.compare_with_guess(guess);
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
        let secret: Word<5> = secret.parse()?;
        let guess: Word<5> = guess.parse()?;
        let res = secret.compare_with_guess(guess);
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
            allowed_guesses: secret.clone(),
            possible_secrets: secret,
        };
        let after = before
            .after_guess("chart".parse()?, "_G__G".parse()?)
            .unwrap();

        assert_eq!(after.possible_secrets, vec!["ghost".parse()?]);
        Ok(())
    }
}
