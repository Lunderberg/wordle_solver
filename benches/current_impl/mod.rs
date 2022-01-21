mod bench_strategy;
mod word_comparison;

use criterion::{BenchmarkId, Criterion};
use wordle::strategy;

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("current_impl");
    group.bench_function("word_comparison", word_comparison::bench::<5>);
    group.finish();

    let mut group = c.benchmark_group("current_strategies");

    group.sample_size(10);

    let test_sizes = [
        (200, 100),
        (10657, 2315), // Size of wordle's guess/secret sizes.
    ];

    test_sizes.iter().for_each(|sizes| {
        group.bench_with_input(
            BenchmarkId::new("minimax", format!("{:?}", sizes)),
            sizes,
            bench_strategy::bench::<_, 5>(strategy::MiniMax),
        );
    });
    group.finish();
}
