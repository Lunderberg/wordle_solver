// -- Cargo.toml --
// [[bench]]
// name = "benchmarks"
// harness = false
//
// [dev-dependencies]
// criterion = {version = "0.3", features=['html_reports']}

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    // BatchSize,
    // Bencher,
};

mod word_comparisons;

fn group_compare_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    group.bench_function(
        "current_impl",
        word_comparisons::current_impl::bench::<5>,
    );
    group.bench_function("bytearray", word_comparisons::bytearray::bench::<5>);
    group.bench_function(
        "string_to_vec",
        word_comparisons::string_to_vector::bench(5),
    );

    group.finish();
}

criterion_group!(benches, group_compare_words);
criterion_main!(benches);
