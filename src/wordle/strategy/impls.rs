use super::HeuristicStrategy;
use crate::{Clue, GameState, Word};

use std::cmp::Reverse;

use itertools::Itertools;

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

    fn fmt(&self, heuristic: &Self::Output) -> String {
        format!("{heuristic}")
    }
}

// Make whatever guess has the most possible clues, which minimizes
// the average size of the next generation's solution space.
pub struct MinimizeMean;

impl<const N: usize> HeuristicStrategy<N> for MinimizeMean {
    type Output = Reverse<usize>;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output {
        Reverse(
            state
                .possible_secrets
                .iter()
                .map(|secret| secret.compare_with_guess(*guess))
                .unique()
                .count(),
        )
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
        *guess
    }

    fn word_options<'a>(&self, state: &'a GameState<N>) -> &'a Vec<Word<N>> {
        &state.possible_secrets
    }
}
