use std::{ffi::OsStr, path::PathBuf};

use criterion::{criterion_group, criterion_main, Criterion};

lazy_static::lazy_static! {
    static ref TRUE: String = resolve_true(which::which("true")
        .unwrap()).to_string_lossy().into_owned();
}

fn resolve_true(p: PathBuf) -> PathBuf {
    // `true` may point to `coreutils`; find the final symlink with correct
    // name. For normal process we could override argv[0], but that doesn't
    // work with wrappers.
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

fn bench_normal_process(c: &mut Criterion) {
    c.bench_function("normal process", |b| b.iter(|| run(&TRUE, &[])));
}

fn bench_sh(c: &mut Criterion) {
    c.bench_function("sh", |b| b.iter(|| run("sh", &["-c", &TRUE])));
}

fn bench_bwrap(c: &mut Criterion) {
    c.bench_function("bwrap", |b| {
        b.iter(|| run("bwrap", &["--ro-bind", "/", "/", &TRUE]))
    });
}

fn bench_bwrap_unshare_all(c: &mut Criterion) {
    c.bench_function("bwrap --unshare-all", |b| {
        b.iter(|| {
            run("bwrap", &["--unshare-all", "--ro-bind", "/", "/", &TRUE])
        })
    });
}

fn bench_unshare_library(c: &mut Criterion) {
    c.bench_function("unshare library", |b| {
        b.iter(|| {
            assert!(unshare::Command::new(&*TRUE)
                .unshare(&[
                    unshare::Namespace::Cgroup,
                    unshare::Namespace::Ipc,
                    unshare::Namespace::Mount,
                    unshare::Namespace::Net,
                    unshare::Namespace::Pid,
                    // no time option
                    unshare::Namespace::User,
                    unshare::Namespace::Uts,
                ])
                .status()
                .unwrap()
                .success())
        })
    });
}

fn bench_unshare(c: &mut Criterion) {
    c.bench_function("unshare", |b| {
        b.iter(|| {
            run(
                "unshare",
                &[
                    "--cgroup", "--ipc", "--mount", "--net", "--pid", "--time",
                    "--user", "--uts", &TRUE,
                ],
            )
        })
    });
}

fn bench_docker(c: &mut Criterion) {
    c.bench_function("docker", |b| {
        b.iter(|| {
            run(
                "docker",
                &["run", "registry.hub.docker.com/library/alpine", "/bin/true"],
            )
        })
    });
}

fn bench_podman(c: &mut Criterion) {
    c.bench_function("podman", |b| {
        b.iter(|| {
            run(
                "podman",
                &["run", "registry.hub.docker.com/library/alpine", "/bin/true"],
            )
        })
    });
}

criterion_group!(
    benches,
    bench_normal_process,
    bench_sh,
    bench_bwrap,
    bench_bwrap_unshare_all,
    bench_unshare_library,
    bench_unshare,
    bench_docker,
    bench_podman,
);
criterion_main!(benches);
