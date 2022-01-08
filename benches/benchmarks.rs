// -- Cargo.toml --
// [[bench]]
// name = "benchmarks"
// harness = false
//
// [dev-dependencies]
// criterion = {version = "0.3", features=['html_reports']}

use criterion::{criterion_group, criterion_main};

mod current_impl;
mod word_comparison;

criterion_group!(
    benches,
    current_impl::benchmark_group,
    word_comparison::benchmark_group
);
criterion_main!(benches);
