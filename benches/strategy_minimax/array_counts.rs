use criterion::{BatchSize, Bencher};

use super::utils::GameStateGenerator;
use wordle::GameState;

pub fn bench<
    CounterType: num::Unsigned + std::cmp::Ord + std::ops::AddAssign + std::marker::Copy,
>(
    bencher: &mut Bencher,
    sizes: &(usize, usize),
) {
    let (num_allowed_guesses, num_possible_secrets) = *sizes;

    // I'd like this to be a const generic, but it will require the
    // generic_const_exprs feature
    // (https://github.com/rust-lang/rust/issues/76560) in order to
    // compute ARR_SIZE.
    const N: usize = 5;
    assert!(num_allowed_guesses >= num_possible_secrets);

    let mut generator = GameStateGenerator::new();

    let setup = move || -> GameState<N> {
        generator.random_state(num_allowed_guesses, num_possible_secrets)
    };

    let routine = |state: &mut GameState<N>| {
        *state
            .allowed_guesses
            .iter()
            .min_by_key(|guess| {
                const ARR_SIZE: usize = 3usize.pow(N as u32);
                let mut counts = [CounterType::zero(); ARR_SIZE];
                state.possible_secrets.iter().for_each(|secret| {
                    let clue = secret.compare_with_guess(**guess);
                    counts[clue.id()] += CounterType::one();
                });

                *counts.iter().max().unwrap()
            })
            .unwrap()
    };

    bencher.iter_batched_ref(setup, routine, BatchSize::SmallInput);
}
