use criterion::{BatchSize, Bencher};

use super::utils::GameStateGenerator;
use wordle::{GameState, Tile};

pub fn bench<const N: usize>(bencher: &mut Bencher, sizes: &(usize, usize)) {
    let (num_allowed_guesses, num_possible_secrets) = *sizes;

    assert!(num_allowed_guesses >= num_possible_secrets);

    let mut generator = GameStateGenerator::new();

    let setup = move || -> GameState<N> {
        generator.random_state(num_allowed_guesses, num_possible_secrets)
    };

    let routine = |state: &mut GameState<N>| {
        state
            .allowed_guesses
            .iter()
            .min_by_key(|guess| {
                let arr_size: usize = 3usize.pow(N as u32);
                let mut counts = vec![0; arr_size];
                state.possible_secrets.iter().for_each(|secret| {
                    let clue = secret.compare_with_guess(**guess);
                    let clue_id = clue
                        .iter()
                        .map(|tile| match tile {
                            Tile::Correct => 0,
                            Tile::WrongPosition => 1,
                            Tile::NotPresentInWord => 2,
                        })
                        .fold(0, |acc, trit| 3 * acc + trit);
                    counts[clue_id] += 1;
                });

                let max_counts: usize = *counts.iter().max().unwrap();
                max_counts
            })
            .unwrap()
            .clone()
    };

    bencher.iter_batched_ref(setup, routine, BatchSize::SmallInput);
}
