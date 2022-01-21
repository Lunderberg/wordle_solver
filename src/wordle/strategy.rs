use super::{Clue, Error, GameState, Word};

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
                let mut counts = vec![0; Clue::<N>::num_clues()];
                state.possible_secrets.iter().for_each(|secret| {
                    let clue = secret.compare_with_guess(**guess);
                    counts[clue.id()] += 1;
                });

                let max_counts: usize = *counts.iter().max().unwrap();
                max_counts
            })
            .unwrap()
            .clone())
    }
}
