//! Memory benchmark harness (docs/improve-memory.md §18.4).
//!
//! Runs the queries of `tests/fixtures/memory_eval.json` through recall and
//! reports recall, precision, cross-project hits and the size of the recall
//! block. The default test uses a fake embedder, so it runs in CI. The ignored
//! test uses a real Ollama server:
//!   CRUSTLY_BENCH_OLLAMA_URL=http://localhost:11434 \
//!   CRUSTLY_BENCH_MODEL=nomic-embed-text \
//!   cargo test --test memory_benchmark_test -- --ignored --nocapture

use async_trait::async_trait;
use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::llm::agent::context::token_count;
use crustly::memory::embed::{
    EmbedError, EmbedPurpose, EmbedSettings, Embedder, OllamaEmbedder, Prefixes,
};
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crustly::memory::project::{id_for_root, ProjectIdentity};
use crustly::memory::semantic::EmbedRuntime;
use crustly::memory::vector::normalize;
use crustly::memory::{MemoryStore, RecallRequest};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

#[derive(Deserialize)]
struct Fixture {
    memories: Vec<FixtureMemory>,
    queries: Vec<FixtureQuery>,
}

#[derive(Deserialize)]
struct FixtureMemory {
    key: String,
    project: String,
    category: String,
    content: String,
}

#[derive(Deserialize)]
struct FixtureQuery {
    project: String,
    text: String,
    expect: Vec<String>,
    forbid: Vec<String>,
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/memory_eval.json");
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

#[derive(Debug, Default)]
struct Metrics {
    recall: f64,
    precision: f64,
    cross_project_hits: usize,
    average_block_tokens: f64,
}

/// Topic fake for CI: three topics, so meaning matches without shared words.
struct TopicEmbedder;

#[async_trait]
impl Embedder for TopicEmbedder {
    fn model_id(&self) -> &str {
        "fake:topics"
    }

    async fn embed(
        &self,
        _purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError> {
        Ok(texts
            .iter()
            .map(|t| {
                let l = t.to_lowercase();
                let v = if ["block", "stall", "load", "slow", "async", "thread", "batch"]
                    .iter()
                    .any(|w| l.contains(w))
                {
                    [1.0, 0.0, 0.0]
                } else if [
                    "database",
                    "postgres",
                    "sqlite",
                    "storage",
                    "pool",
                    "connection",
                ]
                .iter()
                .any(|w| l.contains(w))
                {
                    [0.0, 1.0, 0.0]
                } else {
                    [0.0, 0.0, 1.0]
                };
                // One axis per text, so distinct texts stay distinct (the
                // near-duplicate rule must not merge them); the topic axes
                // carry the meaning.
                let axis = t.bytes().map(usize::from).sum::<usize>() % 97;
                let mut w = v.to_vec();
                w.extend((0..97).map(|i| if i == axis { 1.0 } else { 0.0 }));
                normalize(&w).unwrap()
            })
            .collect())
    }
}

/// Save the fixture into a fresh database, then run every query and score it.
/// Returns metrics for the store as configured (with or without an embedder).
async fn run(embedder: Option<(Arc<dyn Embedder>, EmbedRuntime)>) -> Metrics {
    let fixture = load_fixture();
    let dir = tempfile::tempdir().unwrap();
    let db = Database::connect(&dir.path().join("bench.db"))
        .await
        .unwrap();
    db.run_migrations().await.unwrap();
    let mut store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    let attached = embedder.is_some();
    if let Some((embedder, runtime)) = embedder {
        store = store.attach_embedder(embedder, runtime, false);
    }

    let projects: HashMap<&str, ProjectIdentity> = ["a", "b"]
        .into_iter()
        .map(|name| {
            let root = PathBuf::from(format!("/bench/{name}"));
            let identity = ProjectIdentity {
                id: id_for_root(&root),
                root,
                name: name.to_string(),
                remote_url: None,
            };
            (name, identity)
        })
        .collect();
    for identity in projects.values() {
        store.register_project(identity).await.unwrap();
    }

    // Fixture key -> project, and the short id shown in the block -> key.
    let mut project_of: HashMap<String, String> = HashMap::new();
    let mut key_of_short: HashMap<String, String> = HashMap::new();
    for memory in &fixture.memories {
        let scope = match memory.project.as_str() {
            "global" => MemoryScope::Global,
            name => MemoryScope::Project(projects[name].id.clone()),
        };
        let category = memory.category.parse::<MemoryCategory>().unwrap();
        match store
            .save(NewMemory {
                content: memory.content.clone(),
                scope,
                category,
                source: MemorySource::User,
                importance: 4,
                replaces: None,
                session_id: None,
                expires_at: None,
            })
            .await
            .unwrap()
        {
            SaveOutcome::Created(id) => {
                key_of_short.insert(id.simple().to_string()[..8].to_string(), memory.key.clone());
                project_of.insert(memory.key.clone(), memory.project.clone());
            }
            other => panic!("the fixture must save cleanly: {other:?}"),
        }
    }
    if attached {
        // Give the embeddings a moment to be stored by reindexing synchronously.
        store.reindex(false).await.unwrap();
    }

    let mut recall_sum = 0.0;
    let mut recall_n = 0usize;
    let mut precision_sum = 0.0;
    let mut precision_n = 0usize;
    let mut cross = 0usize;
    let mut tokens_sum = 0usize;
    for query in &fixture.queries {
        let project = &projects[query.project.as_str()];
        let block = store
            .recall(&RecallRequest {
                project,
                message: &query.text,
                previous: None,
                window: 128_000,
                first_request: false,
                root: Path::new("/nonexistent"),
            })
            .await
            .unwrap_or_default();
        tokens_sum += token_count(&block) as usize;

        let selected: Vec<String> = block
            .lines()
            .filter(|l| l.starts_with("- ["))
            .filter_map(|l| {
                let short = l.split("m:").nth(1)?.get(..8)?;
                key_of_short.get(short).cloned()
            })
            .collect();
        let chosen: HashSet<&String> = selected.iter().collect();

        if !query.expect.is_empty() {
            let found = query.expect.iter().filter(|k| chosen.contains(k)).count();
            recall_sum += found as f64 / query.expect.len() as f64;
            recall_n += 1;
        }
        if !selected.is_empty() {
            let relevant = selected.iter().filter(|k| query.expect.contains(k)).count();
            precision_sum += relevant as f64 / selected.len() as f64;
            precision_n += 1;
        }
        for key in &selected {
            let other_project = match project_of.get(key).map(String::as_str) {
                Some("global") | None => false,
                Some(name) => name != query.project,
            };
            if other_project || query.forbid.contains(key) {
                cross += 1;
            }
        }
    }

    Metrics {
        recall: if recall_n > 0 {
            recall_sum / recall_n as f64
        } else {
            0.0
        },
        precision: if precision_n > 0 {
            precision_sum / precision_n as f64
        } else {
            0.0
        },
        cross_project_hits: cross,
        average_block_tokens: tokens_sum as f64 / fixture.queries.len() as f64,
    }
}

fn report(label: &str, m: &Metrics) {
    println!(
        "{label:<22} recall {:.2}  precision {:.2}  cross-project {}  avg block {:.0} tokens",
        m.recall, m.precision, m.cross_project_hits, m.average_block_tokens
    );
}

#[tokio::test]
async fn the_harness_runs_and_meets_the_ci_floor() {
    let keyword = run(None).await;
    let semantic = run(Some((
        Arc::new(TopicEmbedder) as Arc<dyn Embedder>,
        EmbedRuntime {
            query_timeout: Duration::from_secs(2),
            ..EmbedRuntime::default()
        },
    )))
    .await;
    report("keyword only", &keyword);
    report("semantic (fake)", &semantic);

    assert_eq!(
        semantic.cross_project_hits, 0,
        "a memory from another project is never recalled"
    );
    assert!(
        semantic.recall >= 0.8,
        "recall with semantic search: {semantic:?}"
    );
    assert!(
        semantic.recall >= keyword.recall,
        "semantic search must not recall less"
    );
}

#[tokio::test]
#[ignore = "needs a local Ollama server; see the module comment"]
async fn real_embedder_benchmark() {
    let url = std::env::var("CRUSTLY_BENCH_OLLAMA_URL")
        .unwrap_or_else(|_| "http://localhost:11434".into());
    let model = std::env::var("CRUSTLY_BENCH_MODEL").unwrap_or_else(|_| "nomic-embed-text".into());
    let settings = EmbedSettings {
        base_url: url,
        model: model.clone(),
        prefixes: Prefixes::for_model(&model, None, None),
        timeout: Duration::from_secs(30),
        api_key: None,
    };
    let embedder = OllamaEmbedder::new(settings).unwrap();
    let keyword = run(None).await;
    let hybrid = run(Some((
        Arc::new(embedder) as Arc<dyn Embedder>,
        EmbedRuntime {
            min_similarity: 0.35,
            ..EmbedRuntime::default()
        },
    )))
    .await;
    report("keyword only", &keyword);
    report(&format!("hybrid ({model})"), &hybrid);
    assert_eq!(hybrid.cross_project_hits, 0);
}
