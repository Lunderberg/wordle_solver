mod word_comparison;

use criterion::Criterion;

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    group.bench_function("word_comparison", word_comparison::bench::<5>);

    group.finish();
}
