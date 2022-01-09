mod bytearray_class;
mod bytearray_direct;
mod string_to_vector;

use criterion::Criterion;

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    group.noise_threshold(0.1);

    group.bench_function("bytearray_class", bytearray_class::bench::<5>);
    group.bench_function("bytearray_direct", bytearray_direct::bench::<5>);
    group.bench_function("string_to_vec", string_to_vector::bench(5));

    group.finish();
}
