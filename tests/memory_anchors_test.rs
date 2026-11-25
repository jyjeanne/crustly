//! Code anchors of memories (docs/improve-memory.md §23.3; task P6.2).

use chrono::{DateTime, Utc};
use crustly::config::MemoryConfig;
use crustly::db::repository::MemoryRepository;
use crustly::db::Database;
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crustly::memory::project::{id_for_root, ProjectIdentity};
use crustly::memory::MemoryStore;
use std::path::PathBuf;

async fn setup() -> (Database, MemoryStore, ProjectIdentity) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    let root = PathBuf::from("/work/anchors");
    let project = ProjectIdentity {
        id: id_for_root(&root),
        root,
        name: "anchors".into(),
        remote_url: None,
    };
    store.register_project(&project).await.unwrap();
    (db, store, project)
}

async fn save(store: &MemoryStore, project: &ProjectIdentity, content: &str) -> uuid::Uuid {
    match store
        .save(NewMemory {
            content: content.into(),
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
        other => panic!("{other:?}"),
    }
}

fn at(secs: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(secs, 0).unwrap()
}

#[tokio::test]
async fn anchors_are_replaced_listed_and_deleted_with_their_memory() {
    let (db, store, project) = setup().await;
    let repo = MemoryRepository::new(db.pool().clone());
    let id = save(&store, &project, "Reuse the shared pool in `connect_pool`.").await;

    repo.set_anchors(id, &["functions/src/db/connect_pool".into()], at(100))
        .await
        .unwrap();
    repo.set_anchors(
        id,
        &[
            "functions/src/db/connect_pool".into(),
            "functions/src/db/open".into(),
        ],
        at(200),
    )
    .await
    .unwrap();
    let anchors = repo.anchors_of(id).await.unwrap();
    assert_eq!(
        anchors,
        vec![
            ("functions/src/db/connect_pool".to_string(), at(200)),
            ("functions/src/db/open".to_string(), at(200)),
        ]
    );
    assert_eq!(
        repo.anchors_for(&project.id, Utc::now())
            .await
            .unwrap()
            .len(),
        2
    );

    store.forget(id, "user").await.unwrap();
    assert!(repo.anchors_of(id).await.unwrap().is_empty(), "cascade");
}

#[tokio::test]
async fn anchors_of_archived_memories_are_not_listed_for_the_project() {
    let (db, store, project) = setup().await;
    let repo = MemoryRepository::new(db.pool().clone());
    let id = save(&store, &project, "Retry on SQLITE_BUSY.").await;
    repo.set_anchors(id, &["functions/src/db/retry".into()], at(100))
        .await
        .unwrap();
    repo.archive(id, "user", Utc::now()).await.unwrap();
    assert!(repo
        .anchors_for(&project.id, Utc::now())
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        repo.anchors_of(id).await.unwrap().len(),
        1,
        "kept as history"
    );
}
