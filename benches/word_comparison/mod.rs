mod bytearray;
//mod current_impl;
mod string_to_vector;

use criterion::Criterion;

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    //group.bench_function("current_impl", current_impl::bench::<5>);
    group.bench_function("bytearray", bytearray::bench::<5>);
    group.bench_function("string_to_vec", string_to_vector::bench(5));

    group.finish();
}
