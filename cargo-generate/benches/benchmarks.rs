use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Replace this with benchmarks of your actual hot paths.
/// Run with: cargo bench
/// HTML reports are written to target/criterion/.
fn bench_example(c: &mut Criterion) {
    c.bench_function("example", |b| b.iter(|| black_box(0)));
}

criterion_group!(benches, bench_example);
criterion_main!(benches);
