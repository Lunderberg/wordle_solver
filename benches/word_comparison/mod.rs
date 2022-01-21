mod bytearray;
mod bytearray_arr_to_arr_quadratic;
mod bytearray_class_to_class_quadratic;
mod string_to_vector_quadratic;

use criterion::Criterion;

pub fn benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_words");

    group.noise_threshold(0.1);

    group.bench_function("bytearray", bytearray::bench::<5>);
    group.bench_function(
        "bytearray_class_to_class_quadratic",
        bytearray_class_to_class_quadratic::bench::<5>,
    );
    group.bench_function(
        "bytearray_arr_to_arr_quadratic",
        bytearray_arr_to_arr_quadratic::bench::<5>,
    );
    group.bench_function("string_to_vec", string_to_vector_quadratic::bench(5));

    group.finish();
}
