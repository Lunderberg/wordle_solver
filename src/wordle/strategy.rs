use super::{Clue, Error, GameState, Word};

use std::cmp::Reverse;

use itertools::Itertools;

pub trait Strategy<const N: usize> {
    fn make_guess(&self, state: &GameState<N>) -> Result<Word<N>, Error>;

    // Returns all possible sequences of guesses resulting from
    // application of a deterministic strategy.
    fn deterministic_strategy_results(
        &self,
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
    fn make_guess(&self, state: &GameState<N>) -> Result<Word<N>, Error> {
        self.as_ref().make_guess(state)
    }
}

pub trait HeuristicStrategy<const N: usize> {
    type Output: Ord;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output;
    fn word_options<'a>(&self, state: &'a GameState<N>) -> &'a Vec<Word<N>> {
        &state.allowed_guesses
    }
}

impl<H: HeuristicStrategy<N>, const N: usize> Strategy<N> for H {
    fn make_guess(&self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.len() == 0 {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
            Ok(self
                .word_options(state)
                .iter()
                .min_by_key(|guess| self.heuristic(state, guess))
                .unwrap()
                .clone())
        }
    }
}

// Make whatever guess results has the best worst-case scenario.
pub struct MiniMax;

impl<const N: usize> HeuristicStrategy<N> for MiniMax {
    type Output = usize;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output {
        let mut counts = vec![0; Clue::<N>::num_clues()];
        state.possible_secrets.iter().for_each(|secret| {
            let clue = secret.compare_with_guess(*guess);
            counts[clue.id()] += 1;
        });

        let max_counts: usize = *counts.iter().max().unwrap();
        max_counts
    }
}

// Make whatever guess has the most possible clues, which minimizes
// the average size of the next generation's solution space.
pub struct MinimizeMean;

impl<const N: usize> HeuristicStrategy<N> for MinimizeMean {
    type Output = usize;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output {
        state
            .possible_secrets
            .iter()
            .map(|secret| secret.compare_with_guess(*guess))
            .unique()
            .count()
    }
}

// Make whatever guess has the most possible clues, which minimizes
// the average size of the next generation's solution space.
pub struct MinimizeSumSquares;

impl<const N: usize> HeuristicStrategy<N> for MinimizeSumSquares {
    type Output = usize;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output {
        state
            .possible_secrets
            .iter()
            .map(|secret| secret.compare_with_guess(*guess))
            .counts()
            .into_values()
            .map(|c| c * c)
            .sum::<usize>()
    }
}

// Make whatever guess would allow the largest number of words to be
// guessed on the next turn.
pub struct EarlyGuesses;

impl<const N: usize> HeuristicStrategy<N> for EarlyGuesses {
    type Output = Reverse<usize>;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output {
        Reverse(
            state
                .possible_secrets
                .iter()
                .map(|secret| secret.compare_with_guess(*guess))
                .counts()
                .into_values()
                .filter(|&counts| counts == 1)
                .count(),
        )
    }
}

// Guess the first secret worst that is still possible.
pub struct AlphabeticalOrder;

impl<const N: usize> HeuristicStrategy<N> for AlphabeticalOrder {
    type Output = Word<N>;
    fn heuristic(
        &self,
        _state: &GameState<N>,
        guess: &Word<N>,
    ) -> Self::Output {
        guess.clone()
    }

    fn word_options<'a>(&self, state: &'a GameState<N>) -> &'a Vec<Word<N>> {
        &state.possible_secrets
    }
}
