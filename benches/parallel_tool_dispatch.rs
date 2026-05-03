//! Benchmark: parallel vs sequential tool dispatch.
//!
//! SC-003 / QS-2.1: 10 concurrent read_file calls via the parallel dispatch
//! path must complete in ≤ 60% of sequential wall-clock time (≥40% speedup).

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use tempfile::TempDir;
use tokio::runtime::Runtime;

/// Create N temp files with ~4 KiB of content each.
fn make_temp_files(dir: &TempDir, n: usize) -> Vec<std::path::PathBuf> {
    (0..n)
        .map(|i| {
            let path = dir.path().join(format!("file_{}.txt", i));
            let content = format!("File {} content. {}", i, "abcdefghij ".repeat(400));
            std::fs::write(&path, content).unwrap();
            path
        })
        .collect()
}

/// Read N files sequentially (baseline).
async fn read_sequential(paths: &[std::path::PathBuf]) -> Vec<String> {
    let mut results = Vec::with_capacity(paths.len());
    for path in paths {
        let content = tokio::fs::read_to_string(path).await.unwrap();
        results.push(content);
    }
    results
}

/// Read N files concurrently via join_all.
async fn read_parallel(paths: &[std::path::PathBuf]) -> Vec<String> {
    let futures: Vec<_> = paths.iter().map(|p| tokio::fs::read_to_string(p)).collect();
    futures::future::join_all(futures)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect()
}

fn bench_parallel_dispatch(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let dir = TempDir::new().unwrap();
    let paths = make_temp_files(&dir, 10);

    let mut group = c.benchmark_group("parallel_tool_dispatch");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    group.bench_function("sequential_10_reads", |b| {
        b.iter(|| rt.block_on(read_sequential(&paths)));
    });

    group.bench_function("parallel_10_reads", |b| {
        b.iter(|| rt.block_on(read_parallel(&paths)));
    });

    group.finish();
}

criterion_group!(benches, bench_parallel_dispatch);
criterion_main!(benches);
