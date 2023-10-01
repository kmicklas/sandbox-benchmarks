use std::{ffi::OsStr, path::PathBuf};

use criterion::{criterion_group, criterion_main, Criterion};

lazy_static::lazy_static! {
    static ref TRUE: String = resolve_true(which::which("true").unwrap()).to_string_lossy().into_owned();
}

fn resolve_true(p: PathBuf) -> PathBuf {
    // `true` may point to `coreutils`, find the final symlink with correct name
    match std::fs::read_link(&p) {
        Ok(p) if p.file_name() == Some(OsStr::new("true")) => resolve_true(p),
        _ => p,
    }
}

fn run(cmd: &str, args: &[&str]) {
    assert!(std::process::Command::new(cmd)
        .args(args)
        .status()
        .unwrap()
        .success())
}

fn bench_true(c: &mut Criterion) {
    c.bench_function("true", |b| b.iter(|| run(&TRUE, &[])));
}

fn bench_sh_true(c: &mut Criterion) {
    c.bench_function("sh true", |b| b.iter(|| run("sh", &["-c", &TRUE])));
}

fn bench_bwrap_true(c: &mut Criterion) {
    c.bench_function("bwrap true", |b| {
        b.iter(|| run("bwrap", &["--ro-bind", "/", "/", &TRUE]))
    });
}

fn bench_bwrap_unshare_true(c: &mut Criterion) {
    c.bench_function("bwrap true (unshare)", |b| {
        b.iter(|| run("bwrap", &["--unshare-all", "--ro-bind", "/", "/", &TRUE]))
    });
}

fn bench_direct_unshare_true(c: &mut Criterion) {
    c.bench_function("direct unshare true", |b| {
        b.iter(|| {
            assert!(unshare::Command::new(&*TRUE)
                .unshare(&[
                    unshare::Namespace::Mount,
                    unshare::Namespace::Uts,
                    unshare::Namespace::Ipc,
                    unshare::Namespace::User,
                    unshare::Namespace::Pid,
                    unshare::Namespace::Net,
                    unshare::Namespace::Cgroup,
                ])
                .status()
                .unwrap()
                .success())
        })
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
    bench_bwrap_unshare_true,
    bench_direct_unshare_true,
);
criterion_main!(benches);
