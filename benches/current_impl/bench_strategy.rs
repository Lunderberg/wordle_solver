use criterion::{BatchSize, Bencher};
use std::convert::TryInto;

use rand::{Rng, SeedableRng};

use wordle::{GameState, Strategy, Word};

pub fn bench<S: Strategy<N>, const N: usize>(
    strategy: S,
) -> impl FnMut(&mut Bencher, &(usize, usize)) {
    move |bencher: &mut Bencher,
          &(allowed_guess_size, possible_secrets_size)| {
        assert!(allowed_guess_size >= possible_secrets_size);

        let seed = 0;

        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        let mut random_word = move || {
            let letters = (0..N)
                .map(|_| rng.gen_range(0..26))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap();
            Word { letters }
        };

        let mut random_word_list =
            |n: usize| -> Vec<_> { (0..n).map(|_| random_word()).collect() };

        let setup = move || -> GameState<N> {
            let possible_secrets = random_word_list(possible_secrets_size);
            let allowed_guesses =
                random_word_list(allowed_guess_size - possible_secrets_size)
                    .into_iter()
                    .chain(possible_secrets.iter().cloned())
                    .collect();
            GameState {
                made_correct_guess: false,
                possible_secrets,
                allowed_guesses,
            }
        };

        let routine = |state: &mut GameState<N>| strategy.make_guess(state);

        bencher.iter_batched_ref(setup, routine, BatchSize::SmallInput);
    }
}
