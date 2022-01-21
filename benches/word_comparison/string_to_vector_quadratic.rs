use criterion::{BatchSize, Bencher};
use wordle::Tile;

use itertools::Itertools;
use rand::{Rng, SeedableRng};

pub fn bench(word_size: usize) -> impl FnMut(&mut Bencher) {
    move |b: &mut Bencher| {
        let seed = 0;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        let mut random_word = move || -> String {
            (0..word_size)
                .map(|_| -> char {
                    let initial: u32 = 'A'.into();
                    let offset = rng.gen_range(0..26);
                    ((initial as u8) + offset).into()
                })
                .collect()
        };

        let setup = move || (random_word(), random_word());

        let routine =
            |vals: &mut (String, String)| compare_words(&vals.0, &vals.1);

        b.iter_batched_ref(setup, routine, BatchSize::SmallInput);
    }
}

pub enum Error {
    UnequalWordLength,
}

fn compare_words(secret_word: &str, guess: &str) -> Result<Vec<Tile>, Error> {
    if secret_word.len() != guess.len() {
        return Err(Error::UnequalWordLength);
    }

    let mut result = vec![Tile::NotPresentInWord; secret_word.len()];

    secret_word
        .chars()
        .zip(guess.chars())
        .map(|(a, b)| a == b)
        .zip(result.iter_mut())
        .filter_map(|(matching, loc)| matching.then(|| loc))
        .for_each(|loc| *loc = Tile::Correct);

    let mut counts = secret_word
        .chars()
        .zip(result.iter())
        .filter_map(|(c, res)| (*res != Tile::Correct).then(|| c))
        .counts();

    guess
        .chars()
        .zip(result.iter_mut())
        .filter(|(_guess_char, res)| **res != Tile::Correct)
        .for_each(|(guess_char, res)| {
            let remaining = counts.get_mut(&guess_char);
            if let Some(remaining) = remaining {
                if *remaining > 0 {
                    *remaining -= 1;
                    *res = Tile::WrongPosition;
                }
            }
        });

    Ok(result)
}
