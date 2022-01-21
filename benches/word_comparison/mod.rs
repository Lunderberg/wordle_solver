mod bytearray_arr_to_arr;
mod bytearray_class_to_class;
mod bytearray_class_to_class_2pass;
mod string_to_vector;

use criterion::Criterion;

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    group.noise_threshold(0.1);

    group.bench_function(
        "bytearray_class_to_class",
        bytearray_class_to_class::bench::<5>,
    );
    group.bench_function(
        "bytearray_class_to_class_2pass",
        bytearray_class_to_class_2pass::bench::<5>,
    );
    group.bench_function(
        "bytearray_arr_to_arr",
        bytearray_arr_to_arr::bench::<5>,
    );
    group.bench_function("string_to_vec", string_to_vector::bench(5));

    group.finish();
}
