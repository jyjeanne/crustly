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

    let root = project.root.clone();

    let session = Uuid::new_v4();
    let first = store
        .memories_for_path(&project, session, "migrations/x.sql", &root)
        .await
        .unwrap();
    assert_eq!(first.len(), 1);
    let not_sent = store
        .memories_for_path(&project, session, "migrations/x.sql", &root)
        .await
        .unwrap();
    assert_eq!(
        not_sent.len(),
        1,
        "offered again until it is marked as sent"
    );

    store.mark_shown(session, &[id]);
    let again = store
        .memories_for_path(&project, session, "migrations/x.sql", &root)
        .await
        .unwrap();
    assert!(again.is_empty(), "already shown in this session");
    let other_session = store
        .memories_for_path(&project, Uuid::new_v4(), "migrations/x.sql", &root)
        .await
        .unwrap();
    assert_eq!(other_session.len(), 1, "a new session shows it again");
    let unrelated = store
        .memories_for_path(&project, session, "src/main.rs", &root)
        .await
        .unwrap();
    assert!(unrelated.is_empty());
}

/// A project on disk, so paths can be absolute and canonical.
async fn store_and_real_project(dir: &Path) -> (Database, MemoryStore, ProjectIdentity) {
    let db = Database::connect(&dir.join("crustly.db")).await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    let raw_root = dir.join("proj");
    std::fs::create_dir_all(raw_root.join("src").join("tui")).unwrap();
    std::fs::write(
        raw_root.join("src").join("tui").join("app.rs"),
        "fn main() {}",
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("other")).unwrap();
    std::fs::write(dir.join("other").join("app.rs"), "").unwrap();
    let root = std::fs::canonicalize(&raw_root).unwrap();
    let project = ProjectIdentity {
        id: id_for_root(&root),
        root,
        name: "proj".into(),
        remote_url: None,
    };
    store.register_project(&project).await.unwrap();
    (db, store, project)
}

#[tokio::test]
async fn tool_paths_match_when_absolute_or_relative_to_a_subdirectory() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store, project) = store_and_real_project(dir.path()).await;
    let id = save_in_project(&store, &project, "Never block the event loop in the tui.").await;
    store
        .add_paths(id, &["src/tui/**".into()])
        .await
        .unwrap()
        .unwrap();
    let session = Uuid::new_v4();
    let root = project.root.clone();
    let hits = |path: String, working_dir: PathBuf| {
        let store = &store;
        let project = &project;
        async move {
            store
                .memories_for_path(project, session, &path, &working_dir)
                .await
                .unwrap()
                .len()
        }
    };

    let canonical = root.join("src").join("tui").join("app.rs");
    assert_eq!(
        hits(canonical.to_string_lossy().into_owned(), root.clone()).await,
        1
    );
    // The same file through the path as first given, not canonicalised.
    let as_given = dir
        .path()
        .join("proj")
        .join("src")
        .join("tui")
        .join("app.rs");
    assert_eq!(
        hits(as_given.to_string_lossy().into_owned(), root.clone()).await,
        1
    );
    // Forward slashes, relative to a working directory below the root.
    assert_eq!(hits("tui/app.rs".into(), root.join("src")).await, 1);
    assert_eq!(hits("../src/tui/app.rs".into(), root.join("src")).await, 1);
    // A file that does not exist yet (a write) still matches.
    assert_eq!(hits("src/tui/new.rs".into(), root.clone()).await, 1);
    // Outside the project: no match, even with the same relative tail.
    let outside = dir.path().join("other").join("app.rs");
    assert_eq!(
        hits(outside.to_string_lossy().into_owned(), root.clone()).await,
        0
    );
    assert_eq!(hits("../other/app.rs".into(), root.clone()).await, 0);
}

#[tokio::test]
async fn an_absolute_path_in_the_message_brings_its_memory_in() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store, project) = store_and_real_project(dir.path()).await;
    let id = save_in_project(&store, &project, "Never block the event loop in the tui.").await;
    store
        .add_paths(id, &["src/tui/**".into()])
        .await
        .unwrap()
        .unwrap();
    // An unrelated long memory makes the candidates too big for the budget,
    // so only a path match passes the floor.
    save_in_project(&store, &project, &"word ".repeat(80)).await;

    // As a user would type it: not canonicalised (no `\\?\` on Windows).
    let file = dir
        .path()
        .join("proj")
        .join("src")
        .join("tui")
        .join("app.rs");
    let message = format!("why does {} freeze?", file.to_string_lossy());
    let block = store
        .recall(&RecallRequest {
            project: &project,
            message: &message,
            previous: None,
            window: 1000,
            first_request: false,
            root: Path::new("/nonexistent"),
        })
        .await
        .unwrap_or_default();
    assert!(
        block.contains("event loop"),
        "recalled through the path: {block}"
    );
}
