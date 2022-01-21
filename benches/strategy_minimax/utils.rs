use std::convert::TryInto;

use rand::{Rng, SeedableRng};

use wordle::{GameState, Word};

pub struct GameStateGenerator {
    rng: rand_chacha::ChaCha8Rng,
}

impl GameStateGenerator {
    pub fn new() -> Self {
        let seed = 0;
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        Self { rng }
    }

    fn random_word<const N: usize>(&mut self) -> Word<N> {
        let letters = (0..N)
            .map(|_| self.rng.gen_range(0..26))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        Word { letters }
    }

    fn random_word_list<const N: usize>(
        &mut self,
        num_words: usize,
    ) -> Vec<Word<N>> {
        (0..num_words).map(|_| self.random_word()).collect()
    }

    pub fn random_state<const N: usize>(
        &mut self,
        num_allowed_guesses: usize,
        num_possible_secrets: usize,
    ) -> GameState<N> {
        let possible_secrets = self.random_word_list(num_possible_secrets);
        let allowed_guesses = self
            .random_word_list(num_allowed_guesses - num_possible_secrets)
            .into_iter()
            .chain(possible_secrets.iter().cloned())
            .collect();

        GameState {
            allowed_guesses,
            possible_secrets,
        }
    }
}
