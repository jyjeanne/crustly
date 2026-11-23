//! Tests for `MemoryStore`: the save rules of docs/improve-memory.md §8 and
//! recall of §9 (tasks P1.15 and P1.16), on a migrated database.

use crustly::config::MemoryConfig;
use crustly::db::repository::MemoryRepository;
use crustly::db::Database;
use crustly::llm::agent::project_memory::{append_progress, ProgressEntry};
use crustly::memory::model::{
    MemoryCategory, MemoryScope, MemorySource, NewMemory, RejectReason, SaveOutcome,
};
use crustly::memory::project::ProjectIdentity;
use crustly::memory::{MemoryStore, RecallRequest};
use std::path::{Path, PathBuf};

async fn store() -> (Database, MemoryStore) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    (db, store)
}

fn project(id: &str) -> ProjectIdentity {
    ProjectIdentity {
        id: id.to_string(),
        root: PathBuf::from(format!("/work/{id}")),
        name: id.to_string(),
        remote_url: None,
    }
}

fn save_request(scope: MemoryScope, content: &str, source: MemorySource) -> NewMemory {
    NewMemory {
        content: content.to_string(),
        scope,
        category: MemoryCategory::Fact,
        source,
        importance: 3,
        replaces: None,
        session_id: None,
        expires_at: None,
    }
}

fn request<'a>(
    project: &'a ProjectIdentity,
    message: &'a str,
    root: &'a Path,
    first: bool,
) -> RecallRequest<'a> {
    RecallRequest {
        project,
        message,
        previous: None,
        window: 128_000,
        first_request: first,
        root,
    }
}

// ── P1.15: save ──────────────────────────────────────────────────────────

#[tokio::test]
async fn saving_twice_gives_one_memory() {
    let (_db, store) = store().await;
    store.register_project(&project("a")).await.unwrap();
    let scope = MemoryScope::Project("a".into());

    let first = store
        .save(save_request(
            scope.clone(),
            "Uses tokio.",
            MemorySource::Tool,
        ))
        .await
        .unwrap();
    let SaveOutcome::Created(id) = first else {
        panic!("first save must create: {first:?}")
    };
    let second = store
        .save(save_request(scope, "uses   TOKIO", MemorySource::Tool))
        .await
        .unwrap();
    assert_eq!(second, SaveOutcome::Duplicate(id));
}

#[tokio::test]
async fn user_repeats_raise_importance_and_model_repeats_do_not() {
    let (db, store) = store().await;
    let scope = MemoryScope::Global;
    let SaveOutcome::Created(id) = store
        .save(save_request(
            scope.clone(),
            "Prefers Tokio for async.",
            MemorySource::User,
        ))
        .await
        .unwrap()
    else {
        panic!("first save must create");
    };
    let repo = MemoryRepository::new(db.pool().clone());
    let importance = || async { repo.get(id).await.unwrap().unwrap().importance };

    store
        .save(save_request(
            scope.clone(),
            "Prefers Tokio for async.",
            MemorySource::User,
        ))
        .await
        .unwrap();
    assert_eq!(importance().await, 4, "the user's repeat raises importance");

    store
        .save(save_request(
            scope,
            "Prefers Tokio for async.",
            MemorySource::Tool,
        ))
        .await
        .unwrap();
    assert_eq!(importance().await, 4, "the model's repeat does not");
}

#[tokio::test]
async fn a_secret_is_rejected_and_nothing_is_stored() {
    let (_db, store) = store().await;
    let key = ["AKIA", "ABCDEFGHIJKLMNOP"].concat();
    let outcome = store
        .save(save_request(
            MemoryScope::Global,
            &format!("My key is {key}"),
            MemorySource::User,
        ))
        .await
        .unwrap();
    assert!(matches!(
        outcome,
        SaveOutcome::Rejected(RejectReason::Secret { .. })
    ));
    assert!(
        store.take_notices().is_empty(),
        "a rejected save is not announced"
    );
}

#[tokio::test]
async fn invalid_text_is_rejected_with_a_reason() {
    let (_db, store) = store().await;
    let outcome = store
        .save(save_request(MemoryScope::Global, "   ", MemorySource::User))
        .await
        .unwrap();
    assert_eq!(outcome, SaveOutcome::Rejected(RejectReason::Empty));
}

#[tokio::test]
async fn a_model_cannot_replace_a_memory_the_user_saved() {
    let (_db, store) = store().await;
    let SaveOutcome::Created(user_id) = store
        .save(save_request(
            MemoryScope::Global,
            "Prefers short answers",
            MemorySource::User,
        ))
        .await
        .unwrap()
    else {
        panic!("user save must create");
    };

    let mut by_model = save_request(
        MemoryScope::Global,
        "Prefers long answers",
        MemorySource::Tool,
    );
    by_model.replaces = Some(user_id);
    assert_eq!(
        store.save(by_model).await.unwrap(),
        SaveOutcome::Rejected(RejectReason::SavedByUser)
    );

    let mut by_user = save_request(
        MemoryScope::Global,
        "Prefers long answers",
        MemorySource::User,
    );
    by_user.replaces = Some(user_id);
    assert!(matches!(
        store.save(by_user).await.unwrap(),
        SaveOutcome::Created(_)
    ));
}

#[tokio::test]
async fn replacement_must_stay_in_the_same_scope() {
    let (_db, store) = store().await;
    store.register_project(&project("a")).await.unwrap();
    let SaveOutcome::Created(project_id) = store
        .save(save_request(
            MemoryScope::Project("a".into()),
            "Uses MySQL",
            MemorySource::Tool,
        ))
        .await
        .unwrap()
    else {
        panic!("project save must create");
    };

    let mut global = save_request(MemoryScope::Global, "Uses PostgreSQL", MemorySource::Tool);
    global.replaces = Some(project_id);
    assert_eq!(
        store.save(global).await.unwrap(),
        SaveOutcome::Rejected(RejectReason::ScopeMismatch)
    );
}

#[tokio::test]
async fn saves_are_announced_once() {
    let (_db, store) = store().await;
    store.register_project(&project("a")).await.unwrap();
    store
        .save(save_request(
            MemoryScope::Project("a".into()),
            "Uses tokio",
            MemorySource::Tool,
        ))
        .await
        .unwrap();

    let notices = store.take_notices();
    assert_eq!(notices.len(), 1);
    assert!(
        notices[0].starts_with("Remembered (project · fact): Uses tokio [m:"),
        "{}",
        notices[0]
    );
    assert!(store.take_notices().is_empty(), "notices are drained once");
}

// ── P1.16: recall ────────────────────────────────────────────────────────

#[tokio::test]
async fn recall_includes_own_project_and_global_only() {
    let (_db, store) = store().await;
    store.register_project(&project("a")).await.unwrap();
    store.register_project(&project("b")).await.unwrap();
    store
        .save(save_request(
            MemoryScope::Global,
            "Prefers Tokio for async",
            MemorySource::User,
        ))
        .await
        .unwrap();
    store
        .save(save_request(
            MemoryScope::Project("a".into()),
            "Uses PostgreSQL for storage",
            MemorySource::Tool,
        ))
        .await
        .unwrap();
    store
        .save(save_request(
            MemoryScope::Project("b".into()),
            "Uses Redis for caching",
            MemorySource::Tool,
        ))
        .await
        .unwrap();

    let a = project("a");
    let block = store
        .recall(&request(
            &a,
            "which database for storage?",
            Path::new("/nonexistent"),
            false,
        ))
        .await
        .expect("memories are recalled");
    assert!(
        block.contains("PostgreSQL"),
        "own project memory is recalled: {block}"
    );
    assert!(
        block.contains("Tokio"),
        "global memory is recalled: {block}"
    );
    assert!(
        !block.contains("Redis"),
        "another project's memory is never recalled: {block}"
    );
}

#[tokio::test]
async fn recall_is_none_when_nothing_is_stored() {
    let (_db, store) = store().await;
    let a = project("a");
    assert_eq!(
        store
            .recall(&request(&a, "anything", Path::new("/nonexistent"), true))
            .await,
        None
    );
}

#[tokio::test]
async fn recent_work_is_added_only_on_the_first_request() {
    let (_db, store) = store().await;
    store.register_project(&project("a")).await.unwrap();
    store
        .save(save_request(
            MemoryScope::Project("a".into()),
            "Uses tokio",
            MemorySource::Tool,
        ))
        .await
        .unwrap();
    let root = tempfile::tempdir().unwrap();
    append_progress(
        root.path(),
        &ProgressEntry {
            request: "Add CSV export",
            tools: "edit_file src/export.rs",
            result: "export works",
        },
    )
    .unwrap();

    let a = project("a");
    let first = store
        .recall(&request(&a, "carry on", root.path(), true))
        .await
        .unwrap();
    assert!(first.contains("Recent work in this project:"), "{first}");
    assert!(first.contains("Add CSV export"));

    let later = store
        .recall(&request(&a, "carry on", root.path(), false))
        .await
        .unwrap();
    assert!(!later.contains("Recent work"), "{later}");
}

#[tokio::test]
async fn recall_records_what_it_sent() {
    let (db, store) = store().await;
    store.register_project(&project("a")).await.unwrap();
    store
        .save(save_request(
            MemoryScope::Project("a".into()),
            "Uses tokio",
            MemorySource::Tool,
        ))
        .await
        .unwrap();
    let a = project("a");
    store
        .recall(&request(&a, "tokio", Path::new("/nonexistent"), false))
        .await
        .unwrap();

    let count: i64 = sqlx::query_scalar("SELECT recall_count FROM memories LIMIT 1")
        .fetch_one(db.pool())
        .await
        .unwrap();
    assert_eq!(count, 1);
}
