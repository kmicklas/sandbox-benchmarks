use criterion::{criterion_group, criterion_main, Criterion};

fn run_true() {
    assert!(std::process::Command::new("true")
        .status()
        .unwrap()
        .success())
}

fn run_brwap_true() {
    assert!(std::process::Command::new("bwrap")
        .args(["--ro-bind", "/", "/"])
        .arg("true")
        .status()
        .unwrap()
        .success())
}

fn run_brwap_unshare_true() {
    assert!(std::process::Command::new("bwrap")
        .arg("--unshare-all")
        .args(["--ro-bind", "/", "/"])
        .arg("true")
        .status()
        .unwrap()
        .success())
}

fn bench_true(c: &mut Criterion) {
    c.bench_function("true", |b| b.iter(|| run_true()));
}

fn bench_bwrap_true(c: &mut Criterion) {
    c.bench_function("bwrap true", |b| b.iter(|| run_brwap_true()));
}

fn bench_bwrap_unshare_true(c: &mut Criterion) {
    c.bench_function("bwrap unshare true", |b| {
        b.iter(|| run_brwap_unshare_true())
    });
}

criterion_group!(
    benches,
    bench_true,
    bench_bwrap_true,
    bench_bwrap_unshare_true
);
criterion_main!(benches);
