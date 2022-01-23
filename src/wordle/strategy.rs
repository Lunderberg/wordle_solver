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

impl<const N: usize> Strategy<N> for Box<dyn Strategy<N>> {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        self.as_mut().make_guess(state)
    }
}

// Make whatever guess results has the best worst-case scenario.
pub struct MiniMax;

impl<const N: usize> Strategy<N> for MiniMax {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
            // let minimax_map: HashMap<usize, Vec<Word<N>>> =
            //     state.allowed_guesses.iter().cloned().into_group_map_by(
            //         |guess| {
            //             let mut counts = vec![0; Clue::<N>::num_clues()];
            //             state.possible_secrets.iter().for_each(|secret| {
            //                 let clue = secret.compare_with_guess(*guess);
            //                 counts[clue.id()] += 1;
            //             });

            //             let max_counts: usize = *counts.iter().max().unwrap();
            //             max_counts
            //         },
            //     );

            // let minimax_child_size = minimax_map.keys().min().unwrap();
            // println!("Min(max(child_size)): {}", minimax_child_size);
            // minimax_map[minimax_child_size]
            //     .iter()
            //     .for_each(|word| println!("\t{}", word));

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

// Make whatever guess has the most possible clues, which minimizes
// the average size of the next generation's solution space.
pub struct MinimizeMean;

impl<const N: usize> Strategy<N> for MinimizeMean {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
            Ok(state
                .allowed_guesses
                .iter()
                .max_by_key(|guess| {
                    state
                        .possible_secrets
                        .iter()
                        .map(|secret| secret.compare_with_guess(**guess))
                        .unique()
                        .count()
                })
                .unwrap()
                .clone())
        }
    }
}

// Make whatever guess has the most possible clues, which minimizes
// the average size of the next generation's solution space.
pub struct MinimizeSumSquares;

impl<const N: usize> Strategy<N> for MinimizeSumSquares {
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
                    state
                        .possible_secrets
                        .iter()
                        .map(|secret| secret.compare_with_guess(**guess))
                        .counts()
                        .into_values()
                        .map(|c| c * c)
                        .sum::<usize>()
                })
                .unwrap()
                .clone())
        }
    }
}

// Make whatever guess would allow the largest number of words to be
// guessed on the next turn.
pub struct EarlyGuesses;

impl<const N: usize> Strategy<N> for EarlyGuesses {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
            Ok(state
                .allowed_guesses
                .iter()
                .max_by_key(|guess| {
                    state
                        .possible_secrets
                        .iter()
                        .map(|secret| secret.compare_with_guess(**guess))
                        .counts()
                        .into_values()
                        .filter(|&counts| counts == 1)
                        .count()
                })
                .unwrap()
                .clone())
        }
    }
}

// Guess the first secret worst that is still possible.
pub struct AlphabeticalOrder;

impl<const N: usize> Strategy<N> for AlphabeticalOrder {
    fn make_guess(&mut self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
            Ok(state.possible_secrets.iter().min().unwrap().clone())
        }
    }
}
