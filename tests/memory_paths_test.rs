//! Path-scoped memories (docs/improve-memory.md §13.5; tasks P4.2 to P4.4).

use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crustly::memory::project::{id_for_root, ProjectIdentity};
use crustly::memory::{MemoryStore, RecallRequest};
use std::path::{Path, PathBuf};
use uuid::Uuid;

async fn store_and_project(dir: &Path) -> (Database, MemoryStore, ProjectIdentity) {
    let db = Database::connect(&dir.join("crustly.db")).await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    let project = ProjectIdentity {
        id: id_for_root(&PathBuf::from("/work/paths")),
        root: PathBuf::from("/work/paths"),
        name: "paths".into(),
        remote_url: None,
    };
    store.register_project(&project).await.unwrap();
    (db, store, project)
}

async fn save_in_project(store: &MemoryStore, project: &ProjectIdentity, content: &str) -> Uuid {
    match store
        .save(NewMemory {
            content: content.to_string(),
            scope: MemoryScope::Project(project.id.clone()),
            category: MemoryCategory::Lesson,
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
        other => panic!("expected a new memory: {other:?}"),
    }
}

#[tokio::test]
async fn paths_must_be_relative_and_few() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store, project) = store_and_project(dir.path()).await;
    let id = save_in_project(&store, &project, "The event loop must not block.").await;

    assert!(store
        .add_paths(id, &["/etc/passwd".into()])
        .await
        .unwrap()
        .is_err());
    assert!(store
        .add_paths(id, &["../outside/**".into()])
        .await
        .unwrap()
        .is_err());
    let six: Vec<String> = (0..6).map(|i| format!("dir{i}/**")).collect();
    assert!(
        store.add_paths(id, &six).await.unwrap().is_err(),
        "at most five"
    );

    store
        .add_paths(id, &["src/tui/**".into()])
        .await
        .unwrap()
        .unwrap();
    let stored = store.get(id).await.unwrap();
    assert!(stored.is_some());
}

#[tokio::test]
async fn a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store, project) = store_and_project(dir.path()).await;
    let about_tui =
        save_in_project(&store, &project, "Never block the event loop in the tui.").await;
    store
        .add_paths(about_tui, &["src/tui/**".into()])
        .await
        .unwrap()
        .unwrap();
    // An unrelated long memory makes the candidates too big for the budget,
    // so the relevance floor decides.
    save_in_project(&store, &project, &"word ".repeat(80)).await;

    let recall = |message: &'static str| {
        let project = &project;
        let store = &store;
        async move {
            store
                .recall(&RecallRequest {
                    project,
                    message,
                    previous: None,
                    window: 1000,
                    first_request: false,
                    root: Path::new("/nonexistent"),
                })
                .await
                .unwrap_or_default()
        }
    };

    let mentioning = recall("why does src/tui/app.rs freeze?").await;
    assert!(
        mentioning.contains("event loop"),
        "recalled through the path: {mentioning}"
    );

    let unrelated = recall("quarterly billing report slow").await;
    assert!(
        !unrelated.contains("event loop"),
        "no path mentioned, not recalled: {unrelated}"
    );
}

#[tokio::test]
async fn a_path_memory_is_shown_once_per_session() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store, project) = store_and_project(dir.path()).await;
    let id = save_in_project(&store, &project, "Migrations are append-only.").await;
    store
        .add_paths(id, &["migrations/*.sql".into()])
        .await
        .unwrap()
        .unwrap();

    let session = Uuid::new_v4();
    let first = store
        .memories_for_path(&project, session, "migrations/x.sql")
        .await
        .unwrap();
    assert_eq!(first.len(), 1);
    let again = store
        .memories_for_path(&project, session, "migrations/x.sql")
        .await
        .unwrap();
    assert!(again.is_empty(), "already shown in this session");
    let other_session = store
        .memories_for_path(&project, Uuid::new_v4(), "migrations/x.sql")
        .await
        .unwrap();
    assert_eq!(other_session.len(), 1, "a new session shows it again");
    let unrelated = store
        .memories_for_path(&project, session, "src/main.rs")
        .await
        .unwrap();
    assert!(unrelated.is_empty());
}
