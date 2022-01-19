use criterion::{BatchSize, Bencher};
use std::convert::TryInto;

use rand::{Rng, SeedableRng};

use wordle::Word;

pub fn bench<const N: usize>(b: &mut Bencher) {
    let seed = 0;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let mut random_word = move || {
        let letters = (0..N)
            .map(|_| -> char {
                let initial: u32 = 'A'.into();
                let offset = rng.gen_range(0..26);
                ((initial as u8) + offset).into()
            })
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        Word { letters }
    };

    let setup = move || (random_word(), random_word());

    let routine =
        |vals: &mut (Word<N>, Word<N>)| vals.0.compare_with_guess(vals.1);

    b.iter_batched_ref(setup, routine, BatchSize::SmallInput);
}
