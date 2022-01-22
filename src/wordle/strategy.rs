use super::{Clue, Error, GameState, Word};

use itertools::Itertools;

pub trait Strategy<const N: usize> {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error>;

    // Returns
    fn deterministic_strategy_results(
        &mut self,
        initial_state: GameState<N>,
    ) -> Vec<Vec<Word<N>>> {
        let mut final_paths = Vec::new();
        let mut stack = vec![(Vec::new(), initial_state)];

        while stack.len() > 0 {
            let (mut path, state) = stack.pop().unwrap();

            let guess = self.make_guess(&state).unwrap();
            path.push(guess);

            state
                .possible_secrets
                .iter()
                .map(|secret| secret.compare_with_guess(guess))
                .unique()
                .for_each(|clue| {
                    if clue.all_correct() {
                        final_paths.push(path.clone());
                    } else {
                        stack.push((
                            path.clone(),
                            state.after_guess(guess, clue),
                        ))
                    }
                })
        }

        final_paths
    }
}

pub struct MiniMax;

impl<const N: usize> Strategy<N> for MiniMax {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
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
}
