//! Long-term memory recall benchmarks (docs/improve-memory.md §18.5).
//!
//! Measures the brute-force vector search at 1,000 and 10,000 memories of
//! 768 dimensions, and keyword recall through the store. Targets are to be
//! checked on a documented reference machine; these numbers are not
//! guarantees.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::memory::vector::normalize;
use crustly::memory::vector_index::VectorIndex;
use std::hint::black_box;
use uuid::Uuid;

const DIMENSIONS: usize = 768;

/// A deterministic unit vector for memory `i`.
fn vector(i: usize) -> Vec<f32> {
    let raw: Vec<f32> = (0..DIMENSIONS)
        .map(|d| (((i * 31 + d * 17) % 997) as f32 / 997.0) - 0.5)
        .collect();
    normalize(&raw).expect("non-zero vector")
}

fn index_of(size: usize) -> (VectorIndex, Vec<f32>) {
    let entries: Vec<(Uuid, Vec<f32>)> = (0..size).map(|i| (Uuid::new_v4(), vector(i))).collect();
    let index = VectorIndex::from_entries("bench", DIMENSIONS, entries).expect("sizes match");
    (index, vector(7))
}

fn bench_vector_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_search");
    for size in [1_000usize, 10_000] {
        let (index, query) = index_of(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| black_box(index.search(black_box(&query), 20, 0.35)));
        });
    }
    group.finish();
}

fn bench_keyword_recall(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("runtime");
    let dir = tempfile::tempdir().expect("temp dir");
    let (db, store, project) = runtime.block_on(async {
        let db = Database::connect(&dir.path().join("bench.db"))
            .await
            .expect("db");
        db.run_migrations().await.expect("migrations");
        let store = crustly::memory::MemoryStore::new(db.pool().clone(), MemoryConfig::default());
        let root = dir.path().join("project");
        std::fs::create_dir_all(&root).expect("project dir");
        let project = crustly::memory::project::identify(&root);
        store.register_project(&project).await.expect("register");
        for i in 0..1_000 {
            let _ = store
                .save(crustly::memory::model::NewMemory {
                    content: format!("Benchmark memory number {i} about topic {}", i % 37),
                    scope: crustly::memory::model::MemoryScope::Project(project.id.clone()),
                    category: crustly::memory::model::MemoryCategory::Fact,
                    source: crustly::memory::model::MemorySource::User,
                    importance: 3,
                    replaces: None,
                    session_id: None,
                    expires_at: None,
                })
                .await
                .expect("save");
        }
        (db, store, project)
    });
    let mut group = c.benchmark_group("keyword_recall");
    group.bench_function("1000_memories", |b| {
        b.iter(|| {
            runtime.block_on(async {
                black_box(
                    store
                        .recall(&crustly::memory::RecallRequest {
                            project: &project,
                            message: "why is topic 12 slow",
                            previous: None,
                            window: 128_000,
                            first_request: false,
                            root: dir.path(),
                        })
                        .await,
                )
            })
        });
    });
    group.finish();
    drop(db);
}

criterion_group!(benches, bench_vector_search, bench_keyword_recall);
criterion_main!(benches);
