use wordle::{Clue, Tile};

use std::convert::TryInto;
use std::ops;

use criterion::{black_box, BatchSize, Bencher};
use rand::{Rng, SeedableRng};

pub fn bench<const N: usize>(b: &mut Bencher) {
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

    let setup = move || (random_word(), random_word());

    let routine = |(secret_word, guess)| {
        black_box(compare_words::<N>(secret_word, guess))
    };

    b.iter_batched(setup, routine, BatchSize::SmallInput);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Word<const N: usize> {
    pub letters: [u8; N],
}

pub fn compare_words<const N: usize>(
    secret_word: Word<N>,
    guess: Word<N>,
) -> Clue<N> {
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

    Clue { tiles }
}

impl<const N: usize> Word<N> {
    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.letters.iter()
    }
}

impl<const N: usize> ops::Index<usize> for Word<N> {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.letters[i]
    }
}
