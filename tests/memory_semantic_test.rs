//! Semantic recall with a fake embedder (docs/improve-memory.md §18.3 test C,
//! §11.4; tasks P2.7 and P2.9). The fake places texts by topic, so a query
//! can match a memory without sharing a word with it, which is the case
//! keyword search cannot handle.

use async_trait::async_trait;
use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::memory::embed::{EmbedError, EmbedPurpose, Embedder};
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory};
use crustly::memory::project::{id_for_root, ProjectIdentity};
use crustly::memory::semantic::EmbedRuntime;
use crustly::memory::vector::normalize;
use crustly::memory::{MemoryStore, RecallRequest};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Three topics: latency, databases, and everything else.
fn topic_vector(text: &str) -> Vec<f32> {
    let lower = text.to_lowercase();
    let latency = [
        "blocking", "stall", "load", "latency", "slow", "threads", "async",
    ];
    let database = ["database", "postgres", "sql", "storage"];
    let v = if latency.iter().any(|w| lower.contains(w)) {
        [1.0, 0.0, 0.0]
    } else if database.iter().any(|w| lower.contains(w)) {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    normalize(&v).unwrap()
}

struct FakeEmbedder {
    /// Calls that fail before the embedder starts to work.
    failures_left: AtomicUsize,
    /// When true, every call fails.
    broken: bool,
}

impl FakeEmbedder {
    fn working() -> Arc<Self> {
        Arc::new(Self {
            failures_left: AtomicUsize::new(0),
            broken: false,
        })
    }
}

#[async_trait]
impl Embedder for FakeEmbedder {
    fn model_id(&self) -> &str {
        "fake:topics"
    }

    async fn embed(
        &self,
        _purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError> {
        if self.broken {
            return Err(EmbedError::Transport("down".into()));
        }
        if self.failures_left.load(Ordering::SeqCst) > 0 {
            self.failures_left.fetch_sub(1, Ordering::SeqCst);
            return Err(EmbedError::Transport("warming up".into()));
        }
        Ok(texts.iter().map(|t| topic_vector(t)).collect())
    }
}

fn fast_runtime() -> EmbedRuntime {
    EmbedRuntime {
        query_timeout: Duration::from_secs(2),
        batch_size: 16,
        min_similarity: 0.35,
        retry_delays: vec![Duration::from_millis(1); 3],
    }
}

fn project(root: &Path) -> ProjectIdentity {
    ProjectIdentity {
        id: id_for_root(root),
        root: root.to_path_buf(),
        name: "p".into(),
        remote_url: None,
    }
}

async fn database(dir: &Path) -> Database {
    let db = Database::connect(&dir.join("crustly.db")).await.unwrap();
    db.run_migrations().await.unwrap();
    db
}

async fn save(store: &MemoryStore, scope: MemoryScope, content: &str) {
    store
        .save(NewMemory {
            content: content.to_string(),
            scope,
            category: MemoryCategory::Lesson,
            source: MemorySource::User,
            importance: 4,
            replaces: None,
            session_id: None,
            expires_at: None,
        })
        .await
        .unwrap();
}

/// Wait until `count` memories have vectors, or give up after a few seconds.
async fn wait_until_indexed(store: &MemoryStore, count: i64) {
    for _ in 0..200 {
        if let Some((_, indexed, _)) = store.embedding_status().await.unwrap() {
            if indexed >= count {
                return;
            }
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("the memories were not embedded in time");
}

async fn recall(store: &MemoryStore, project: &ProjectIdentity, message: &str) -> String {
    recall_in_window(store, project, message, 128_000).await
}

async fn recall_in_window(
    store: &MemoryStore,
    project: &ProjectIdentity,
    message: &str,
    window: u32,
) -> String {
    store
        .recall(&RecallRequest {
            project,
            message,
            previous: None,
            window,
            first_request: false,
            root: Path::new("/nonexistent"),
        })
        .await
        .unwrap_or_default()
}

#[tokio::test]
async fn a_memory_is_recalled_by_meaning_without_a_shared_word() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(FakeEmbedder::working(), fast_runtime());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "Avoid blocking operations inside Tokio runtime threads.",
    )
    .await;
    wait_until_indexed(&store, 1).await;

    let block = recall(&store, &project, "Why does my server stall under load?").await;
    assert!(
        block.contains("blocking operations"),
        "recalled by meaning: {block}"
    );
}

#[tokio::test]
async fn an_unrelated_memory_below_the_threshold_is_not_recalled() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(FakeEmbedder::working(), fast_runtime());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "The database uses PostgreSQL.",
    )
    .await;
    // A long, unrelated memory makes the candidates too big for the budget.
    // Only then does the relevance floor decide (§9.3: when everything fits,
    // everything is taken).
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        &"word ".repeat(80),
    )
    .await;
    wait_until_indexed(&store, 2).await;

    let block = recall_in_window(
        &store,
        &project,
        "Why does my server stall under load?",
        1000,
    )
    .await;
    assert!(
        !block.contains("PostgreSQL"),
        "a different topic is below the threshold: {block}"
    );
}

#[tokio::test]
async fn when_the_embedder_is_down_keyword_recall_still_works() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let broken = Arc::new(FakeEmbedder {
        failures_left: AtomicUsize::new(0),
        broken: true,
    });
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(broken, fast_runtime());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "Uses cargo clippy before commits.",
    )
    .await;

    let block = recall(&store, &project, "run clippy please").await;
    assert!(
        block.contains("cargo clippy"),
        "keyword recall carries on: {block}"
    );
}

#[tokio::test]
async fn a_failed_embedding_is_retried_and_then_stored() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let flaky = Arc::new(FakeEmbedder {
        failures_left: AtomicUsize::new(2),
        broken: false,
    });
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(flaky, fast_runtime());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "Blocking calls stall the runtime.",
    )
    .await;

    wait_until_indexed(&store, 1).await;
}

#[tokio::test]
async fn reindex_embeds_memories_saved_before_the_embedder_was_configured() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    {
        let store = MemoryStore::new(
            database(dir.path()).await.pool().clone(),
            MemoryConfig::default(),
        );
        store.register_project(&project).await.unwrap();
        save(
            &store,
            MemoryScope::Project(project.id.clone()),
            "Blocking calls stall the runtime.",
        )
        .await;
    }
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(FakeEmbedder::working(), fast_runtime());
    let (embedded, _) = store.reindex(false).await.unwrap();
    assert!(embedded <= 1, "the worker may already have it");
    wait_until_indexed(&store, 1).await;
    let status = store.embedding_status().await.unwrap().unwrap();
    assert_eq!((status.1, status.2), (1, 1));
}

#[tokio::test]
async fn two_memories_saying_the_same_thing_are_recalled_once() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    // Global memories are not archived as near-duplicates on save (that check
    // is per project), so both reach recall, which must collapse them.
    .with_embedder(FakeEmbedder::working(), fast_runtime());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Global,
        "Blocking calls stall the runtime.",
    )
    .await;
    save(
        &store,
        MemoryScope::Global,
        "Slow blocking work stalls threads.",
    )
    .await;
    wait_until_indexed(&store, 2).await;

    let block = recall(&store, &project, "why is the server slow under load").await;
    let lines = block.lines().filter(|l| l.starts_with("- [")).count();
    assert_eq!(lines, 1, "near-duplicates collapse to one: {block}");
}

#[test]
fn the_embeddings_settings_build_the_right_provider_or_none() {
    use crustly::config::MemoryEmbeddingsConfig;
    use crustly::memory::embedder_from_config;

    let off = MemoryEmbeddingsConfig::default();
    assert!(
        embedder_from_config(&off).unwrap().is_none(),
        "provider none is keyword-only"
    );

    let ollama = MemoryEmbeddingsConfig {
        provider: "ollama".into(),
        ..MemoryEmbeddingsConfig::default()
    };
    let (embedder, runtime) = embedder_from_config(&ollama)
        .unwrap()
        .expect("ollama is built");
    assert_eq!(embedder.model_id(), "ollama:nomic-embed-text");
    assert_eq!(runtime.batch_size, 16);

    let bad_url = MemoryEmbeddingsConfig {
        provider: "ollama".into(),
        base_url: "localhost:11434".into(),
        ..MemoryEmbeddingsConfig::default()
    };
    assert!(
        embedder_from_config(&bad_url).is_err(),
        "a URL without a scheme is refused"
    );
}

#[test]
fn bad_embeddings_settings_fall_back_to_defaults_at_load() {
    use crustly::config::{MemoryConfig, MemoryEmbeddingsConfig};

    let memory = MemoryConfig {
        embeddings: MemoryEmbeddingsConfig {
            provider: "carrier-pigeon".into(),
            min_similarity: 7.0,
            batch_size: 0,
            query_timeout_ms: 0,
            ..MemoryEmbeddingsConfig::default()
        },
        ..MemoryConfig::default()
    };
    let memory = memory.sanitized();
    assert_eq!(memory.embeddings.provider, "none");
    assert!((memory.embeddings.min_similarity - 0.35).abs() < 1e-6);
    assert_eq!(memory.embeddings.batch_size, 16);
    assert_eq!(memory.embeddings.query_timeout_ms, 1500);
}

/// Every text gets the same vector, so any two memories are near-duplicates.
struct SameVectorEmbedder;

#[async_trait]
impl Embedder for SameVectorEmbedder {
    fn model_id(&self) -> &str {
        "fake:same"
    }

    async fn embed(
        &self,
        _purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError> {
        Ok(texts.iter().map(|_| vec![1.0, 0.0, 0.0]).collect())
    }
}

#[tokio::test]
async fn one_of_two_near_duplicates_is_archived_not_deleted() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(
        Arc::new(SameVectorEmbedder) as Arc<dyn Embedder>,
        fast_runtime(),
    );
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "Uses Tokio for async work.",
    )
    .await;
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "We use the Tokio runtime.",
    )
    .await;

    for _ in 0..200 {
        let active = store.list(&project, false).await.unwrap();
        if active.len() == 1 {
            // Which copy stays depends on creation order, which is only as
            // fine as one second here; the rule is that exactly one stays.
            assert!(
                active[0].content.starts_with("Uses Tokio")
                    || active[0].content.starts_with("We use"),
                "one of the two stays: {active:?}"
            );
            let all = store.list(&project, true).await.unwrap();
            assert_eq!(all.len(), 2, "the other one is archived, not deleted");
            return;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("the near-duplicate was not archived in time");
}

/// The vector stored for `id`, once there is one equal to `expected`.
async fn wait_for_vector(
    db: &Database,
    project: &ProjectIdentity,
    id: uuid::Uuid,
    expected: &[f32],
) {
    let repo = crustly::db::repository::MemoryRepository::new(db.pool().clone());
    for _ in 0..200 {
        let vectors = repo
            .vectors_for(&project.id, "fake:topics", chrono::Utc::now())
            .await
            .unwrap();
        if vectors
            .iter()
            .any(|(v_id, v)| *v_id == id && v.as_slice() == expected)
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("the memory was not embedded with the expected vector in time");
}

#[tokio::test]
async fn an_edited_memory_is_embedded_again_from_its_new_text() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let db = database(dir.path()).await;
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default())
        .with_embedder(FakeEmbedder::working(), fast_runtime());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "Deploys migrate the database first.",
    )
    .await;
    let id = store.list(&project, false).await.unwrap()[0].id;
    wait_for_vector(&db, &project, id, &topic_vector("database")).await;

    store
        .edit(id, "Deploys are slow under load.", "user")
        .await
        .unwrap()
        .unwrap();
    wait_for_vector(&db, &project, id, &topic_vector("slow")).await;
}

#[tokio::test]
async fn an_approved_memory_is_embedded() {
    use crustly::db::repository::memory::InsertMemory;
    use crustly::memory::model::MemoryStatus;
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let db = database(dir.path()).await;
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default())
        .with_embedder(FakeEmbedder::working(), fast_runtime());
    store.register_project(&project).await.unwrap();
    let id = uuid::Uuid::new_v4();
    crustly::db::repository::MemoryRepository::new(db.pool().clone())
        .insert(
            &InsertMemory {
                id,
                scope: MemoryScope::Project(project.id.clone()),
                category: MemoryCategory::Lesson,
                content: "Blocking calls stall the runtime.".into(),
                content_hash: format!("pending-{id}"),
                source: MemorySource::Extracted,
                importance: 3,
                status: MemoryStatus::Pending,
                session_id: None,
                expires_at: None,
                now: chrono::Utc::now(),
            },
            "extractor",
        )
        .await
        .unwrap();

    store.approve(id).await.unwrap().unwrap();
    wait_for_vector(&db, &project, id, &topic_vector("blocking")).await;
}

/// With embeddings on, a weak keyword match the semantic search did not
/// return must not outrank a clear semantic match when only one fits.
#[tokio::test]
async fn a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget() {
    let dir = tempfile::tempdir().unwrap();
    let project = project(dir.path());
    let store = MemoryStore::new(
        database(dir.path()).await.pool().clone(),
        MemoryConfig::default(),
    )
    .with_embedder(FakeEmbedder::working(), fast_runtime());
    store.register_project(&project).await.unwrap();
    let scope = MemoryScope::Project(project.id.clone());
    // Shares the word "server" with the request, but not its topic.
    save(
        &store,
        scope.clone(),
        "The server config lives in config/server.toml.",
    )
    .await;
    // Shares no word with the request, but its topic.
    save(
        &store,
        scope,
        "Avoid blocking operations inside Tokio runtime threads.",
    )
    .await;
    wait_until_indexed(&store, 2).await;

    // A budget of 35 tokens: room for one of the two.
    let block = recall_in_window(
        &store,
        &project,
        "Why does the server stall under load?",
        560,
    )
    .await;
    assert!(block.contains("blocking operations"), "{block}");
    assert!(!block.contains("server config"), "{block}");
}
