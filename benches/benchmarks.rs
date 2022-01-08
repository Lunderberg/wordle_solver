// -- Cargo.toml --
// [[bench]]
// name = "benchmarks"
// harness = false
//
// [dev-dependencies]
// criterion = {version = "0.3", features=['html_reports']}

use std::convert::TryInto;

use criterion::{
    criterion_group, criterion_main, BatchSize, Bencher, Criterion,
};
use rand::{Rng, SeedableRng};

use wordle::Word;

fn bench_compare_words<const N: usize>(b: &mut Bencher) {
    use wordle::compare_words;

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
        |vals: &mut (Word<N>, Word<N>)| compare_words::<N>(vals.0, vals.1);

    b.iter_batched_ref(setup, routine, BatchSize::SmallInput);
}

fn group_compare_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    group.bench_function("current_impl", bench_compare_words::<5>);

    group.finish();
}

criterion_group!(benches, group_compare_words);
criterion_main!(benches);
