use super::{MultiStrategy, Strategy};

use std::convert::TryInto;

use rand::Rng;

#[derive(Debug)]
pub enum Error {
    IncorrectStringLength,
    InvalidString(String),
    NoDictionaryFile,
    WordListReadError(std::io::Error),
    NoWordsRemaining,
    NotTileChar(char),
    IncorrectNumberOfWords,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tile {
    Correct,
    WrongPosition,
    NotPresentInWord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Word<const N: usize> {
    pub letters: [u8; N],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Clue<const N: usize> {
    pub tiles: [Tile; N],
}

#[derive(Debug, Clone)]
pub struct GameState<const N: usize> {
    pub made_correct_guess: bool,
    pub allowed_guesses: Vec<Word<N>>,
    pub possible_secrets: Vec<Word<N>>,
}

#[derive(Debug, Clone)]
pub struct MultiGameState<const N: usize, const GAMES: usize> {
    pub games: [GameState<N>; GAMES],
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

    pub fn from_id(mut id: usize) -> Self {
        let mut tiles = [Tile::NotPresentInWord; N];

        std::iter::repeat_with(move || {
            let tile_id = id % 3;
            id /= 3;
            match tile_id {
                0 => Tile::Correct,
                1 => Tile::WrongPosition,
                2 => Tile::NotPresentInWord,
                _ => panic!("Math is broken"),
            }
        })
        .zip(tiles.iter_mut().rev())
        .for_each(|(val, out)| *out = val);

        Self { tiles }
    }
}

impl<const N: usize> GameState<N> {
    pub fn is_finished(&self) -> bool {
        self.made_correct_guess
    }

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
        let made_correct_guess =
            self.made_correct_guess || observed_result.all_correct();
        Self {
            made_correct_guess,
            allowed_guesses: self.allowed_guesses.clone(),
            possible_secrets: secret,
        }
    }

    pub fn simulate_strategy<'a, S: Strategy<N>>(
        &self,
        secret_word: Word<N>,
        strategy: &'a S,
    ) -> impl Iterator<Item = Result<(Option<(Word<N>, Clue<N>)>, Self), Error>> + 'a
    {
        std::iter::successors(
            Some(Ok((None, self.clone()))),
            move |res_state| {
                if let Ok((_prev_clue, state)) = res_state {
                    (!state.is_finished()).then(|| {
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

    pub fn random_secret<T: Rng>(&self, rng: &mut T) -> Word<N> {
        self.possible_secrets[rng.gen_range(0..self.possible_secrets.len())]
    }
}

impl<const N: usize, const GAMES: usize> MultiGameState<N, GAMES> {
    pub fn new(single: GameState<N>) -> Self {
        let games = (0..GAMES)
            .map(|_| single.clone())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { games }
    }

    pub fn is_finished(&self) -> bool {
        self.games.iter().all(|game| game.is_finished())
    }

    pub fn after_guess(&self, guess: Word<N>, clues: [Clue<N>; GAMES]) -> Self {
        let games = self
            .games
            .iter()
            .zip(clues.iter())
            .map(|(game, clue)| game.after_guess(guess, *clue))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { games }
    }

    pub fn simulate_strategy<'a, S: MultiStrategy<N, GAMES>>(
        &self,
        secret_words: [Word<N>; GAMES],
        strategy: &'a S,
    ) -> impl Iterator<
        Item = Result<(Option<(Word<N>, [Clue<N>; GAMES])>, Self), Error>,
    > + 'a {
        std::iter::successors(
            Some(Ok((None, self.clone()))),
            move |res_state| {
                if let Ok((_prev_clue, state)) = res_state {
                    (!state.is_finished()).then(|| {
                        let guess = strategy.make_guess(state)?;
                        let clues = secret_words
                            .iter()
                            .map(|secret_word| {
                                secret_word.compare_with_guess(guess)
                            })
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap();
                        let new_state = state.after_guess(guess, clues);
                        Ok((Some((guess, clues)), new_state))
                    })
                } else {
                    None
                }
            },
        )
    }

    pub fn estimate_difficulty(&self, secret_words: [Word<N>; 4]) -> usize {
        secret_words
            .iter()
            .map(|guess| {
                let game_state = self.clone();
                let clues = secret_words
                    .iter()
                    .map(|secret| secret.compare_with_guess(*guess))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                game_state.after_guess(*guess, clues)
            })
            .map(|state| {
                state
                    .games
                    .iter()
                    .map(|game| game.possible_secrets.len())
                    .product::<usize>()
            })
            .max()
            .unwrap()
    }

    pub fn random_secret<T: Rng>(&self, rng: &mut T) -> [Word<N>; GAMES] {
        self.games
            .iter()
            .map(|game| game.random_secret(rng))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
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
            made_correct_guess: false,
            allowed_guesses: secret.clone(),
            possible_secrets: secret,
        };
        let after = before.after_guess("chart".parse()?, "_G__G".parse()?);

        assert_eq!(after.possible_secrets, vec!["ghost".parse()?]);
        Ok(())
    }

    #[test]
    fn test_clue_id() {
        use std::collections::HashSet;
        let mut all_clues = HashSet::new();
        for id in 0..243 {
            let clue: Clue<5> = Clue::from_id(id);
            let roundtrip_id = clue.id();
            assert_eq!(id, roundtrip_id);
            all_clues.insert(clue);
        }

        assert_eq!(all_clues.len(), 243);
    }
}
