//! Export, import and `--from-notes` against real databases and files
//! (docs/improve-memory.md §15.1).

use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crustly::memory::project::{id_for_root, identify, ProjectIdentity};
use crustly::memory::transfer::{export, import, import_from_notes, ExportScope};
use crustly::memory::MemoryStore;
use std::path::PathBuf;

async fn store() -> (Database, MemoryStore) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    (db, store)
}

fn project_at(root: &std::path::Path) -> ProjectIdentity {
    ProjectIdentity {
        id: id_for_root(root),
        root: root.to_path_buf(),
        name: root.file_name().unwrap().to_string_lossy().into_owned(),
        remote_url: None,
    }
}

async fn save(
    store: &MemoryStore,
    scope: MemoryScope,
    content: &str,
    source: MemorySource,
) -> SaveOutcome {
    store
        .save(NewMemory {
            content: content.to_string(),
            scope,
            category: MemoryCategory::Convention,
            source,
            importance: 4,
            replaces: None,
            session_id: None,
            expires_at: None,
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn export_then_import_recreates_the_memories_in_a_new_database() {
    let root = tempfile::tempdir().unwrap();
    let (_db, source) = store().await;
    let project = project_at(root.path());
    source.register_project(&project).await.unwrap();
    save(
        &source,
        MemoryScope::Project(project.id.clone()),
        "Run clippy before committing",
        MemorySource::User,
    )
    .await;
    save(
        &source,
        MemoryScope::Global,
        "Prefers Tokio",
        MemorySource::User,
    )
    .await;

    let file = export(&source, ExportScope::All).await.unwrap();
    assert_eq!(file.memories.len(), 2);
    assert!(file.memories.iter().all(|m| m.status == "active"));
    let json = serde_json::to_string(&file).unwrap();
    assert!(!json.contains("vector"), "vectors are never exported");

    let (_db2, target) = store().await;
    let report = import(&target, &file).await.unwrap();
    assert_eq!(report.created, 2, "{report:?}");

    // Importing the same file again creates nothing new.
    let again = import(&target, &file).await.unwrap();
    assert_eq!(again.created, 0);
    assert_eq!(again.duplicates, 2);
    let imported = target.list(&project, false).await.unwrap();
    assert!(imported
        .iter()
        .any(|m| m.content == "Run clippy before committing"));
}

#[tokio::test]
async fn export_scope_limits_what_is_written() {
    let root = tempfile::tempdir().unwrap();
    let (_db, store) = store().await;
    let project = project_at(root.path());
    store.register_project(&project).await.unwrap();
    save(
        &store,
        MemoryScope::Project(project.id.clone()),
        "Project only fact",
        MemorySource::User,
    )
    .await;
    save(
        &store,
        MemoryScope::Global,
        "Global fact",
        MemorySource::User,
    )
    .await;

    let global = export(&store, ExportScope::Global).await.unwrap();
    assert_eq!(global.memories.len(), 1);
    assert_eq!(global.memories[0].content, "Global fact");
}

#[tokio::test]
async fn an_unknown_export_version_is_refused() {
    let (_db, store) = store().await;
    let mut file = export(&store, ExportScope::All).await.unwrap();
    file.version = 99;
    assert!(import(&store, &file).await.is_err());
}

#[tokio::test]
async fn from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first() {
    let dir = tempfile::tempdir().unwrap();
    let crustly_dir = dir.path().join(".crustly");
    std::fs::create_dir_all(&crustly_dir).unwrap();
    let notes = crustly_dir.join("memory.md");
    let secret = ["AKIA", "ABCDEFGHIJKLMNOP"].concat();
    let original = format!(
        "# Notes\n\nKept intro.\n\n## Memories\n- Prefers tabs\n- Key is {secret}\n- Uses cargo fmt\n"
    );
    std::fs::write(&notes, &original).unwrap();

    let (_db, store) = store().await;
    let project = identify(dir.path());
    let (report, backup) = import_from_notes(&store, &project, &notes, "20261123-100000")
        .await
        .unwrap();

    assert_eq!(report.created, 2, "{report:?}");
    assert_eq!(
        report.rejected.len(),
        1,
        "the secret bullet is reported: {report:?}"
    );
    let backup = backup.expect("a backup is written");
    assert_eq!(
        std::fs::read_to_string(backup).unwrap(),
        original,
        "the backup is the original"
    );

    let left = std::fs::read_to_string(&notes).unwrap();
    assert!(
        left.contains("Kept intro."),
        "text outside the section is kept"
    );
    assert!(!left.contains("Prefers tabs") && !left.contains("Uses cargo fmt"));
    assert!(
        left.contains(&secret),
        "the rejected bullet stays in the file"
    );
    assert!(
        left.contains("## Memories"),
        "the heading stays while a bullet remains"
    );

    let saved = store.list(&project, false).await.unwrap();
    assert_eq!(saved.len(), 2);
}

#[tokio::test]
async fn from_notes_without_a_file_does_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store) = store().await;
    let project = identify(dir.path());
    let (report, backup) = import_from_notes(
        &store,
        &project,
        &PathBuf::from(dir.path()).join("missing.md"),
        "20261123-100000",
    )
    .await
    .unwrap();
    assert_eq!(report.created, 0);
    assert!(backup.is_none());
}
