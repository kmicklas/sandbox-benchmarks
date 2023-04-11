use criterion::{criterion_group, criterion_main, Criterion};

fn run_true() {
    assert!(std::process::Command::new("true")
        .status()
        .unwrap()
        .success())
}

fn true_benchmark(c: &mut Criterion) {
    c.bench_function("true", |b| b.iter(|| run_true()));
}

criterion_group!(benches, true_benchmark);
criterion_main!(benches);
