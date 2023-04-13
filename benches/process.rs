use criterion::{criterion_group, criterion_main, Criterion};

fn run(cmd: &str, args: &[&str]) {
    assert!(std::process::Command::new(cmd)
        .args(args)
        .status()
        .unwrap()
        .success())
}

fn bench_true(c: &mut Criterion) {
    c.bench_function("true", |b| b.iter(|| run("true", &[])));
}

fn bench_sh_true(c: &mut Criterion) {
    c.bench_function("sh true", |b| b.iter(|| run("sh", &["-c", "true"])));
}

fn bench_bwrap_true(c: &mut Criterion) {
    c.bench_function("bwrap true", |b| {
        b.iter(|| run("bwrap", &["--ro-bind", "/", "/", "true"]))
    });
}

fn bench_bwrap_unshare_true(c: &mut Criterion) {
    c.bench_function("bwrap true (unshare)", |b| {
        b.iter(|| run("bwrap", &["--unshare-all", "--ro-bind", "/", "/", "true"]))
    });
}

// TODO:
// * unshare
// * systemd-nspawn
// * docker

criterion_group!(
    benches,
    bench_true,
    bench_sh_true,
    bench_bwrap_true,
    bench_bwrap_unshare_true
);
criterion_main!(benches);
