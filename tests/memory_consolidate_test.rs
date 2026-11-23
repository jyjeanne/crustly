//! Consolidation (docs/improve-memory.md §8.3, §13; tasks P4.5 to P4.7).

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use crustly::config::MemoryConfig;
use crustly::db::repository::memory::InsertMemory;
use crustly::db::repository::MemoryRepository;
use crustly::db::Database;
use crustly::llm::provider::{
    ContentBlock, LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    StopReason, TokenUsage,
};
use crustly::memory::embed::{EmbedError, EmbedPurpose, Embedder};
use crustly::memory::model::{
    MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome,
};
use crustly::memory::project::{id_for_root, ProjectIdentity};
use crustly::memory::semantic::EmbedRuntime;
use crustly::memory::MemoryStore;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

fn now() -> DateTime<Utc> {
    Utc::now()
}

fn project() -> ProjectIdentity {
    let root = PathBuf::from("/work/consolidate");
    ProjectIdentity {
        id: id_for_root(&root),
        root,
        name: "consolidate".into(),
        remote_url: None,
    }
}

async fn setup() -> (Database, MemoryStore, ProjectIdentity) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    let project = project();
    store.register_project(&project).await.unwrap();
    (db, store, project)
}

/// Insert a memory directly, with its creation time, so tests can age it.
async fn insert_at(
    db: &Database,
    project: &ProjectIdentity,
    category: MemoryCategory,
    content: &str,
    created: DateTime<Utc>,
    expires: Option<DateTime<Utc>>,
) -> Uuid {
    let repo = MemoryRepository::new(db.pool().clone());
    let id = Uuid::new_v4();
    repo.insert(
        &InsertMemory {
            id,
            scope: MemoryScope::Project(project.id.clone()),
            category,
            content: content.to_string(),
            content_hash: format!("hash-{id}"),
            source: MemorySource::User,
            importance: 3,
            status: MemoryStatus::Active,
            session_id: None,
            expires_at: expires,
            now: created,
        },
        "user",
    )
    .await
    .unwrap();
    id
}

#[tokio::test]
async fn an_expired_memory_is_archived_on_apply_and_only_reported_before() {
    let (db, store, project) = setup().await;
    let id = insert_at(
        &db,
        &project,
        MemoryCategory::Fact,
        "Build on Friday only",
        now(),
        Some(now() - Duration::days(1)),
    )
    .await;

    let report = store.consolidate(&project, false).await.unwrap();
    assert_eq!(report.expired, 1);
    assert!(
        store.get(id).await.unwrap().unwrap().status == MemoryStatus::Active,
        "report only"
    );

    store.consolidate(&project, true).await.unwrap();
    assert_eq!(
        store.get(id).await.unwrap().unwrap().status,
        MemoryStatus::Archived
    );
}

#[tokio::test]
async fn old_episodes_are_archived_and_recent_ones_kept() {
    let (db, store, project) = setup().await;
    let old = insert_at(
        &db,
        &project,
        MemoryCategory::Episode,
        "Old session",
        now() - Duration::days(200),
        None,
    )
    .await;
    let recent = insert_at(
        &db,
        &project,
        MemoryCategory::Episode,
        "Recent session",
        now() - Duration::days(5),
        None,
    )
    .await;

    let report = store.consolidate(&project, true).await.unwrap();
    assert_eq!(report.episodes_archived, 1);
    assert_eq!(
        store.get(old).await.unwrap().unwrap().status,
        MemoryStatus::Archived
    );
    assert_eq!(
        store.get(recent).await.unwrap().unwrap().status,
        MemoryStatus::Active
    );
}

#[tokio::test]
async fn a_year_unused_fact_is_listed_for_review_and_never_archived() {
    let (db, store, project) = setup().await;
    let old_fact = insert_at(
        &db,
        &project,
        MemoryCategory::Fact,
        "Uses an old API",
        now() - Duration::days(400),
        None,
    )
    .await;
    let old_pref = insert_at(
        &db,
        &project,
        MemoryCategory::Preference,
        "Prefers tabs",
        now() - Duration::days(400),
        None,
    )
    .await;

    let report = store.consolidate(&project, true).await.unwrap();
    let listed: Vec<Uuid> = report.stale.iter().map(|m| m.id).collect();
    assert!(listed.contains(&old_fact));
    assert!(!listed.contains(&old_pref), "preferences never go stale");
    assert_eq!(
        store.get(old_fact).await.unwrap().unwrap().status,
        MemoryStatus::Active
    );
}

/// Every text gets a vector at the given cosine to the first unit axis.
struct FixedEmbedder;

#[async_trait]
impl Embedder for FixedEmbedder {
    fn model_id(&self) -> &str {
        "fake:fixed"
    }

    async fn embed(
        &self,
        _purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError> {
        Ok(texts.iter().map(|_| vec![1.0, 0.0]).collect())
    }
}

#[tokio::test]
async fn near_duplicates_in_the_merge_band_are_merged_on_apply() {
    let (db, store, project) = setup().await;
    let store = store.attach_embedder(Arc::new(FixedEmbedder), EmbedRuntime::default(), false);
    let older = match store
        .save(NewMemory {
            content: "Run the tests in CI.".into(),
            scope: MemoryScope::Project(project.id.clone()),
            category: MemoryCategory::Convention,
            source: MemorySource::User,
            importance: 3,
            replaces: None,
            session_id: None,
            expires_at: None,
        })
        .await
        .unwrap()
    {
        SaveOutcome::Created(id) => id,
        other => panic!("{other:?}"),
    };
    let newer = match store
        .save(NewMemory {
            content: "CI runs the test suite.".into(),
            scope: MemoryScope::Project(project.id.clone()),
            category: MemoryCategory::Convention,
            source: MemorySource::User,
            importance: 3,
            replaces: None,
            session_id: None,
            expires_at: None,
        })
        .await
        .unwrap()
    {
        SaveOutcome::Created(id) => id,
        other => panic!("{other:?}"),
    };
    // Cosine 0.85 between the two: inside the merge band (0.80 to 0.92).
    let repo = MemoryRepository::new(db.pool().clone());
    repo.put_embedding(older, "fake:fixed", &[1.0, 0.0], now())
        .await
        .unwrap();
    repo.put_embedding(
        newer,
        "fake:fixed",
        &[0.85, (1.0f32 - 0.85 * 0.85).sqrt()],
        now(),
    )
    .await
    .unwrap();

    let report = store.consolidate(&project, false).await.unwrap();
    assert_eq!(report.merge_candidates.len(), 1, "{report:?}");
    assert_eq!(report.merged, 0, "report only");

    let applied = store.consolidate(&project, true).await.unwrap();
    assert_eq!(applied.merged, 1);
    // Both were created in the same second, so which one is "older" is decided
    // by id. Exactly one is superseded, by the other, and the history keeps the link.
    let a = store.get(older).await.unwrap().unwrap();
    let b = store.get(newer).await.unwrap().unwrap();
    let (superseded, keeper) = if a.status == MemoryStatus::Superseded {
        (a, b)
    } else {
        (b, a)
    };
    assert_eq!(superseded.status, MemoryStatus::Superseded);
    assert_eq!(keeper.status, MemoryStatus::Active);
    assert_eq!(
        superseded.superseded_by,
        Some(keeper.id),
        "history keeps the link"
    );
}

/// A model that answers every request with the same text.
struct FixedAnswer(&'static str);

#[async_trait]
impl Provider for FixedAnswer {
    async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse> {
        Ok(LLMResponse {
            id: "r".into(),
            model: "fixed".into(),
            content: vec![ContentBlock::Text {
                text: self.0.to_string(),
            }],
            stop_reason: Some(StopReason::EndTurn),
            usage: TokenUsage {
                input_tokens: 0,
                output_tokens: 0,
            },
            cache_metrics: None,
            perf_metrics: None,
        })
    }

    async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream> {
        unimplemented!("consolidation does not stream")
    }

    fn name(&self) -> &str {
        "fixed"
    }

    fn default_model(&self) -> &str {
        "fixed-model"
    }

    fn supported_models(&self) -> Vec<String> {
        vec!["fixed-model".into()]
    }

    fn context_window(&self, _model: &str) -> Option<u32> {
        Some(128_000)
    }

    fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
        0.0
    }
}

#[tokio::test]
async fn three_episodes_become_one_lesson_with_its_sources() {
    let (db, _store, project) = setup().await;
    let config = crustly::config::MemoryConfig {
        extraction: crustly::config::MemoryExtractionConfig {
            enabled: true,
            ..Default::default()
        },
        ..MemoryConfig::default()
    };
    let store = MemoryStore::new(db.pool().clone(), config).with_extraction(
        Arc::new(FixedAnswer(
            r#"{"lesson": "Keep blocking work off the async runtime."}"#,
        )),
        "fixed-model".into(),
    );
    for i in 0..3 {
        insert_at(
            &db,
            &project,
            MemoryCategory::Episode,
            &format!("Session {i} on blocking calls"),
            now() - Duration::days(i),
            None,
        )
        .await;
    }

    let report = store.consolidate(&project, true).await.unwrap();
    assert_eq!(report.lessons_written, 1, "{report:?}");
    let lessons: Vec<String> = store
        .list(&project, false)
        .await
        .unwrap()
        .into_iter()
        .filter(|m| m.category == MemoryCategory::Lesson)
        .map(|m| m.content)
        .collect();
    assert_eq!(
        lessons,
        vec!["Keep blocking work off the async runtime.".to_string()]
    );
}
