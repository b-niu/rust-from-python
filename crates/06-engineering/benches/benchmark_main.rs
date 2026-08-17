use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stage06_engineering::normalize_slice;

fn bench_normalize(c: &mut Criterion) {
    let mut data = vec![1.5f32, -0.3, 0.4, 0.8, -1.2, 2.0];
    c.bench_function("normalize_slice_small", |b| {
        b.iter(|| {
            normalize_slice(black_box(&mut data));
        })
    });
}

criterion_group!(benches, bench_normalize);
criterion_main!(benches);
