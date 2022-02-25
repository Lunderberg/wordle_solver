use super::{MultiStrategy, Strategy};
use crate::{Error, MultiGameState, Word};

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
