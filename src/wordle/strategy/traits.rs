use crate::{Error, GameState, MultiGameState, Word};

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

        while !stack.is_empty() {
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

pub trait MultiStrategy<const N: usize, const GAMES: usize> {
    fn make_guess(
        &self,
        state: &MultiGameState<N, GAMES>,
    ) -> Result<Word<N>, Error>;
}

impl<const N: usize, const GAMES: usize> MultiStrategy<N, GAMES>
    for Box<dyn MultiStrategy<N, GAMES>>
{
    fn make_guess(
        &self,
        state: &MultiGameState<N, GAMES>,
    ) -> Result<Word<N>, Error> {
        self.as_ref().make_guess(state)
    }
}

pub trait HeuristicStrategy<const N: usize> {
    type Output: Ord;
    fn heuristic(&self, state: &GameState<N>, guess: &Word<N>) -> Self::Output;
    fn word_options<'a>(&self, state: &'a GameState<N>) -> &'a Vec<Word<N>> {
        &state.allowed_guesses
    }
    fn fmt(&self, _heuristic: &Self::Output) -> String {
        return "".to_string();
    }
}

impl<H: HeuristicStrategy<N>, const N: usize> Strategy<N> for H {
    fn make_guess(&self, state: &GameState<N>) -> Result<Word<N>, Error> {
        if state.possible_secrets.is_empty() {
            Err(Error::NoWordsRemaining)
        } else if state.possible_secrets.len() == 1 {
            Ok(state.possible_secrets[0])
        } else {
            Ok(*self
                .word_options(state)
                .iter()
                .min_by_key(|guess| self.heuristic(state, guess))
                .unwrap())
        }
    }
}
