use std::collections::HashSet;

use super::{HeuristicStrategy, MultiStrategy, Strategy};
use crate::{Error, MultiGameState, Word};

// Solve each puzzle in order.  Information gained when solving
// earlier puzzles is tracked in order to apply to later puzzles, but
// the first unsolved puzzle is considered when determining the next
// guess.
pub struct MultiSequential<S: Strategy<N>, const N: usize> {
    single: S,
}

impl<S: Strategy<N>, const N: usize> MultiSequential<S, N> {
    pub fn new(single: S) -> Self {
        Self { single }
    }
}

impl<S: Strategy<N>, const N: usize, const GAMES: usize> MultiStrategy<N, GAMES>
    for MultiSequential<S, N>
{
    fn make_guess(
        &self,
        state: &MultiGameState<N, GAMES>,
    ) -> Result<Word<N>, Error> {
        Ok(state
            .games
            .iter()
            .find(|game| !game.is_finished())
            .map(|game| self.single.make_guess(game))
            .ok_or(Error::NoWordsRemaining)??)
    }
}

// When making a guess, first select the puzzle that is furthest from
// being solved, then make the best guess for that puzzle.
pub struct WorkOnWorst<S: Strategy<N>, const N: usize> {
    single: S,
}

impl<S: Strategy<N>, const N: usize> WorkOnWorst<S, N> {
    pub fn new(single: S) -> Self {
        Self { single }
    }
}

impl<S: Strategy<N>, const N: usize, const GAMES: usize> MultiStrategy<N, GAMES>
    for WorkOnWorst<S, N>
{
    fn make_guess(
        &self,
        state: &MultiGameState<N, GAMES>,
    ) -> Result<Word<N>, Error> {
        state.find_concluding_guess().map_or_else(
            || {
                state
                    .games
                    .iter()
                    .filter(|game| !game.is_finished())
                    .max_by_key(|game| game.possible_secrets.len())
                    .map(|game| self.single.make_guess(game))
                    .unwrap()
            },
            |guess| Ok(guess),
        )
    }
}

//
pub struct MinimizeWorstHeuristic<S: HeuristicStrategy<N>, const N: usize> {
    single: S,
}

impl<S: HeuristicStrategy<N>, const N: usize> MinimizeWorstHeuristic<S, N> {
    pub fn new(single: S) -> Self {
        Self { single }
    }
}

impl<S: HeuristicStrategy<N>, const N: usize, const GAMES: usize>
    MultiStrategy<N, GAMES> for MinimizeWorstHeuristic<S, N>
{
    fn make_guess(
        &self,
        state: &MultiGameState<N, GAMES>,
    ) -> Result<Word<N>, Error> {
        state.find_concluding_guess().map_or_else(
            || {
                let option_set = state
                    .games
                    .iter()
                    .map(|game| game.possible_secrets.iter())
                    .flatten()
                    .collect::<HashSet<_>>();

                state
                    .games
                    .iter()
                    .map(|game| game.allowed_guesses.iter())
                    .flatten()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .min_by_key(|guess| {
                        let ret = (
                            self.multi_heuristic(state, guess),
                            !option_set.contains(guess),
                        );
                        ret
                    })
                    .map(|x| *x)
                    .ok_or(Error::NoWordsRemaining)
            },
            |guess| Ok(guess),
        )
    }
}

impl<S: HeuristicStrategy<N>, const N: usize> MinimizeWorstHeuristic<S, N> {
    fn multi_heuristic<const GAMES: usize>(
        &self,
        state: &MultiGameState<N, GAMES>,
        guess: &Word<N>,
    ) -> S::Output {
        state
            .games
            .iter()
            .map(|game| self.single.heuristic(game, guess))
            .max()
            .unwrap()
    }
}
