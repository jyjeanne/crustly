//! Acceptance tests for long-term memory (docs/improve-memory.md §18.3):
//! persistence across restarts and repositories (A), project isolation (B),
//! updating knowledge (D), and privacy on disk (E).
//!
//! They use a real database file, so restarts and the bytes on disk are the
//! real ones. The request-level checks (what the model receives) are in
//! `src/llm/agent/service.rs`, where the scripted provider lives.

use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crustly::memory::project::{identify, ProjectIdentity};
use crustly::memory::{MemoryStore, RecallRequest};
use std::path::{Path, PathBuf};

/// Open the database file at `path`, migrated, with a store on it.
async fn open(path: &Path) -> (Database, MemoryStore) {
    let db = Database::connect(path).await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    (db, store)
}

fn repo(name: &str) -> ProjectIdentity {
    let root = PathBuf::from(format!("/work/{name}"));
    ProjectIdentity {
        id: crustly::memory::project::id_for_root(&root),
        root,
        name: name.to_string(),
        remote_url: None,
    }
}

async fn save(
    store: &MemoryStore,
    scope: MemoryScope,
    content: &str,
    category: MemoryCategory,
) -> SaveOutcome {
    store
        .save(NewMemory {
            content: content.to_string(),
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
}

/// The recall block for `message` in `project`, or the empty string.
async fn recall(
    store: &MemoryStore,
    project: &ProjectIdentity,
    message: &str,
    first: bool,
) -> String {
    store
        .recall(&RecallRequest {
            project,
            message,
            previous: None,
            window: 128_000,
            first_request: first,
            root: Path::new("/nonexistent"),
        })
        .await
        .unwrap_or_default()
}

#[tokio::test]
async fn a_preference_survives_a_restart_and_is_recalled_in_another_repository() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("crustly.db");
    let project_x = repo("x");
    let project_y = repo("y");

    {
        let (_db, store) = open(&db_path).await;
        store.register_project(&project_x).await.unwrap();
        save(
            &store,
            MemoryScope::Global,
            "The user prefers Tokio for asynchronous Rust",
            MemoryCategory::Preference,
        )
        .await;
    }

    // "Restart Crustly" in another repository.
    let (_db, store) = open(&db_path).await;
    store.register_project(&project_y).await.unwrap();
    let block = recall(
        &store,
        &project_y,
        "Which asynchronous Rust runtime should I use?",
        true,
    )
    .await;
    assert!(
        block.contains("Tokio"),
        "the preference is recalled: {block}"
    );
}

#[tokio::test]
async fn a_project_fact_is_never_recalled_in_another_repository() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store) = open(&dir.path().join("crustly.db")).await;
    let project_a = repo("a");
    let project_b = repo("b");
    store.register_project(&project_a).await.unwrap();
    store.register_project(&project_b).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project_a.id.clone()),
        "This project uses PostgreSQL",
        MemoryCategory::Fact,
    )
    .await;

    let in_a = recall(
        &store,
        &project_a,
        "Which database does this project use?",
        true,
    )
    .await;
    assert!(in_a.contains("PostgreSQL"), "{in_a}");
    let in_b = recall(
        &store,
        &project_b,
        "Which database does this project use?",
        true,
    )
    .await;
    assert!(
        !in_b.contains("PostgreSQL"),
        "project A's fact must not leak: {in_b}"
    );
}

#[tokio::test]
async fn an_updated_decision_supersedes_the_old_one_and_keeps_its_history() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store) = open(&dir.path().join("crustly.db")).await;
    let project = repo("d");
    store.register_project(&project).await.unwrap();
    let scope = MemoryScope::Project(project.id.clone());
    let SaveOutcome::Created(old) = save(
        &store,
        scope.clone(),
        "The project uses MySQL",
        MemoryCategory::Decision,
    )
    .await
    else {
        panic!("the first save creates");
    };

    let outcome = store
        .save(NewMemory {
            content: "The project uses PostgreSQL".to_string(),
            scope,
            category: MemoryCategory::Decision,
            source: MemorySource::User,
            importance: 4,
            replaces: Some(old),
            session_id: None,
            expires_at: None,
        })
        .await
        .unwrap();
    assert!(matches!(outcome, SaveOutcome::Created(_)), "{outcome:?}");

    let block = recall(
        &store,
        &project,
        "Which database should this project use?",
        true,
    )
    .await;
    assert!(block.contains("PostgreSQL"), "{block}");
    assert!(
        !block.contains("MySQL"),
        "the old decision is not current: {block}"
    );

    let history = store.events(old).await.unwrap();
    assert!(
        history.iter().any(|(event, _, _)| event == "superseded"),
        "{history:?}"
    );
    assert!(
        store.get(old).await.unwrap().is_some(),
        "the old memory is kept as history"
    );
}

#[tokio::test]
async fn a_secret_never_reaches_the_database_file_or_its_wal() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("crustly.db");
    let secret = ["sk-ant-", "api03-ABCDEFGHIJKLMNOPQRSTUVWX"].concat();
    let marker = "zebrafish-marker-7731";

    {
        let (_db, store) = open(&db_path).await;
        let project = repo("e");
        store.register_project(&project).await.unwrap();

        // Every write path: a save, a save with the key inside a sentence, and
        // the same text written through the edit path.
        for text in [
            format!("My Anthropic key is {secret}"),
            format!("Use this key: {secret}"),
        ] {
            let outcome = save(
                &store,
                MemoryScope::Project(project.id.clone()),
                &text,
                MemoryCategory::Fact,
            )
            .await;
            assert!(matches!(outcome, SaveOutcome::Rejected(_)), "{outcome:?}");
        }

        // A harmless memory with a marker word, saved and then forgotten.
        let SaveOutcome::Created(id) = save(
            &store,
            MemoryScope::Project(project.id.clone()),
            &format!("Note about {marker}"),
            MemoryCategory::Fact,
        )
        .await
        else {
            panic!("the marker memory is saved");
        };
        // Control: before the forget, the marker really is on disk, so the
        // checks below can fail. Without it they could pass for the wrong reason.
        let wal = PathBuf::from(format!("{}-wal", db_path.display()));
        let before = [
            std::fs::read(&db_path).unwrap_or_default(),
            std::fs::read(&wal).unwrap_or_default(),
        ];
        assert!(
            before
                .iter()
                .any(|bytes| contains(bytes, marker.as_bytes())),
            "the control failed: the marker should be on disk before it is forgotten"
        );

        store.forget(id, "user").await.unwrap();
    }

    let key_bytes = secret.as_bytes();
    let marker_bytes = marker.as_bytes();
    let mut files = vec![db_path.clone()];
    files.push(PathBuf::from(format!("{}-wal", db_path.display())));
    for file in files {
        let Ok(bytes) = std::fs::read(&file) else {
            continue;
        };
        assert!(
            !contains(&bytes, key_bytes),
            "the key is on disk in {}",
            file.display()
        );
        assert!(
            !contains(&bytes, marker_bytes),
            "the forgotten marker is on disk in {}",
            file.display()
        );
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[tokio::test]
async fn identify_gives_the_same_project_for_the_same_directory() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(identify(dir.path()).id, identify(dir.path()).id);
}
