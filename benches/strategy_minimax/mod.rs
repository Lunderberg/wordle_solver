mod array_counts;
mod hashmap_clue;
mod hashmap_int_id;
mod utils;
mod vec_counts;

use criterion::{BenchmarkId, Criterion};

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("strategy_minimax");

    //group.noise_threshold(0.1);

    let test_sizes = [
        (200, 100),
        // (10657, 2315), // Size of wordle's guess/secret sizes.
    ];

    test_sizes.iter().for_each(|sizes| {
        group.bench_with_input(
            BenchmarkId::new("hashmap_counts", format!("{:?}", sizes)),
            sizes,
            hashmap_clue::bench::<5>,
        );

        group.bench_with_input(
            BenchmarkId::new("hashmap_int_id", format!("{:?}", sizes)),
            sizes,
            hashmap_int_id::bench::<5>,
        );

        group.bench_with_input(
            BenchmarkId::new("array_counts", format!("{:?}", sizes)),
            sizes,
            array_counts::bench,
        );

        group.bench_with_input(
            BenchmarkId::new("vec_counts", format!("{:?}", sizes)),
            sizes,
            vec_counts::bench::<5>,
        );
    });

    group.finish();
}
