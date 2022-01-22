use super::Strategy;

#[derive(Debug)]
pub enum Error {
    IncorrectStringLength,
    InvalidString(String),
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Word<const N: usize> {
    pub letters: [u8; N],
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

        let mut excess_letters = [0_u8; 26];
        for i in 0..N {
            if guess[i] != self[i] {
                excess_letters[self[i] as usize] += 1;
            }
        }

        for i in 0..N {
            tiles[i] = if guess[i] == self[i] {
                Tile::Correct
            } else if excess_letters[guess[i] as usize] > 0 {
                excess_letters[guess[i] as usize] -= 1;
                Tile::WrongPosition
            } else {
                Tile::NotPresentInWord
            }
        }

        Clue { tiles }
    }
}

impl<const N: usize> Clue<N> {
    pub fn all_correct(&self) -> bool {
        self.iter().all(|&tile| tile == Tile::Correct)
    }

    pub fn num_clues() -> usize {
        3_usize.pow(N as u32)
    }

    pub fn id(&self) -> usize {
        self.iter()
            .map(|tile| match tile {
                Tile::Correct => 0,
                Tile::WrongPosition => 1,
                Tile::NotPresentInWord => 2,
            })
            .fold(0, |acc, trit| 3 * acc + trit)
    }
}

impl<const N: usize> GameState<N> {
    pub fn after_guess(
        &self,
        guess: Word<N>,
        observed_result: Clue<N>,
    ) -> Self {
        let secret = self
            .possible_secrets
            .iter()
            .filter(|secret| {
                secret.compare_with_guess(guess) == observed_result
            })
            .copied()
            .collect();
        Self {
            allowed_guesses: self.allowed_guesses.clone(),
            possible_secrets: secret,
        }
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
                        let new_state = state.after_guess(guess, clue);
                        Ok((Some((guess, clue)), new_state))
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
        let after = before.after_guess("chart".parse()?, "_G__G".parse()?);

        assert_eq!(after.possible_secrets, vec!["ghost".parse()?]);
        Ok(())
    }
}
