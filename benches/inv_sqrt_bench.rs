#[macro_use]
extern crate criterion;
extern crate raytracer;

use criterion::Criterion;
use criterion::black_box;

use raytracer::util::fast_inv_sqrt;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("inv_sqrt 20", |b| b.iter(|| fast_inv_sqrt(black_box(20.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
