use wordle::Tile;

use std::convert::TryInto;

use criterion::{black_box, BatchSize, Bencher};
use rand::{Rng, SeedableRng};

pub fn bench<const N: usize>(b: &mut Bencher) {
    let seed = 0;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let mut random_word = move || -> [u8; N] {
        (0..N)
            .map(|_| rng.gen_range(0..26))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap()
    };

    let setup = move || (random_word(), random_word());

    let routine = |(secret_word, guess)| {
        black_box(compare_words::<N>(secret_word, guess))
    };

    b.iter_batched(setup, routine, BatchSize::SmallInput);
}

pub fn compare_words<const N: usize>(
    secret_word: [u8; N],
    guess: [u8; N],
) -> [Tile; N] {
    let mut tiles = [Tile::NotPresentInWord; N];

    for i in 0..N {
        tiles[i] = if guess[i] == secret_word[i] {
            Tile::Correct
        } else {
            let num_occurrences =
                secret_word.iter().filter(|c| **c == guess[i]).count();
            let i_occurrence =
                guess.iter().take(i).filter(|c| **c == guess[i]).count();
            if i_occurrence < num_occurrences {
                Tile::WrongPosition
            } else {
                Tile::NotPresentInWord
            }
        }
    }

    tiles
}
