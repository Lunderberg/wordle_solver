use super::{Error, GameState, Word};

use itertools::Itertools;

pub trait Strategy<const N: usize> {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error>;
}

pub struct MiniMax;

impl<const N: usize> Strategy<N> for MiniMax {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            return Err(Error::NoWordsRemaining);
        }
        Ok(state
            .allowed_guesses
            .iter()
            .min_by_key(|guess| {
                state
                    .possible_secrets
                    .iter()
                    .map(|secret| secret.compare_with_guess(**guess))
                    .counts()
                    .into_values()
                    .max()
                    .unwrap()
            })
            .unwrap()
            .clone())
    }
}
