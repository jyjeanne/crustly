//! Code knowledge from an okf-rs bundle (docs/improve-memory.md §23.8;
//! tasks P6.4 to P6.7), against a scripted okf-mcp, so CI needs no okf-rs.
//! `real_okf_mcp_answers_in_json` runs against a real one when
//! `CRUSTLY_TEST_OKF_MCP` names its executable.

use async_trait::async_trait;
use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::mcp::client::McpToolDef;
use crustly::memory::code_knowledge::{CodeKnowledge, Connector, Detection, OkfServer};
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crustly::memory::project::{id_for_root, ProjectIdentity};
use crustly::memory::{MemoryStore, RecallRequest};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

/// One concept of the scripted bundle.
#[derive(Clone)]
struct Concept {
    id: &'static str,
    title: &'static str,
    file: &'static str,
    lines: (u64, u64),
    calls: Vec<&'static str>,
}

fn fixture() -> Vec<Concept> {
    vec![
        Concept {
            id: "functions/src/db/pool/connect_pool",
            title: "connect_pool",
            file: "src/db/pool.rs",
            lines: (10, 20),
            calls: vec![],
        },
        Concept {
            id: "functions/src/server/start_server",
            title: "start_server",
            file: "src/server.rs",
            lines: (1, 30),
            calls: vec!["functions/src/db/pool/connect_pool"],
        },
        Concept {
            id: "functions/src/db/File/open_handle",
            title: "open_handle",
            file: "src/db/file.rs",
            lines: (1, 5),
            calls: vec![],
        },
        Concept {
            id: "functions/src/net/Socket/open_handle",
            title: "open_handle",
            file: "src/net.rs",
            lines: (1, 5),
            calls: vec![],
        },
        // A test helper with the same name: never an anchor target.
        Concept {
            id: "functions/tests/pool_test/connect_pool",
            title: "connect_pool",
            file: "tests/pool_test.rs",
            lines: (1, 9),
            calls: vec![],
        },
    ]
}

/// A scripted okf-mcp: answers the JSON forms of `search_ranked`,
/// `explore` and `graph` from `concepts`, after `delay`. With `text_only`
/// it is an older server with no JSON results.
#[derive(Clone)]
struct Scripted {
    concepts: Arc<Mutex<Vec<Concept>>>,
    delay: Duration,
    text_only: bool,
}

impl Scripted {
    fn new() -> Self {
        Self {
            concepts: Arc::new(Mutex::new(fixture())),
            delay: Duration::ZERO,
            text_only: false,
        }
    }

    fn connector(&self) -> Connector {
        let me = self.clone();
        Box::new(move || {
            let me = me.clone();
            Box::pin(async move { Ok(Box::new(me) as Box<dyn OkfServer>) })
        })
    }

    fn remove(&self, id: &str) {
        self.concepts.lock().unwrap().retain(|c| c.id != id);
    }

    fn reference(c: &Concept) -> Value {
        json!({
            "id": c.id, "title": c.title, "type": "Rust Function",
            "location": { "file": c.file, "start_line": c.lines.0, "end_line": c.lines.1 }
        })
    }
}

#[async_trait]
impl OkfServer for Scripted {
    async fn tools(&mut self) -> anyhow::Result<Vec<McpToolDef>> {
        let format = if self.text_only {
            json!({})
        } else {
            json!({ "format": { "type": "string", "enum": ["text", "json"] } })
        };
        let mut graph = format.clone();
        graph["relation"] = json!({ "enum": if self.text_only { json!(["callers"]) } else { json!(["callers", "file"]) } });
        let tool = |name: &str, properties: Value| McpToolDef {
            name: name.to_string(),
            description: None,
            input_schema: Some(json!({ "type": "object", "properties": properties })),
        };
        Ok(vec![
            tool("search_ranked", format.clone()),
            tool("explore", format),
            tool("graph", graph),
        ])
    }

    async fn call(&mut self, tool: &str, args: Value) -> anyhow::Result<String> {
        tokio::time::sleep(self.delay).await;
        let concepts = self.concepts.lock().unwrap().clone();
        let find = |id: &str| concepts.iter().find(|c| c.id == id).cloned();
        let doc = match (tool, args["relation"].as_str()) {
            ("search_ranked", _) => {
                let query = args["query"].as_str().unwrap_or("").to_lowercase();
                let hits: Vec<Value> = concepts
                    .iter()
                    .filter(|c| c.title.to_lowercase().contains(&query))
                    .map(|c| json!({ "id": c.id, "title": c.title, "type": "Rust Function", "score": 1.0 }))
                    .collect();
                json!({ "schema": "okf-search-ranked/1", "query": query, "hits": hits })
            }
            ("explore", _) => {
                let id = args["query"].as_str().unwrap_or("");
                let Some(c) = find(id) else {
                    return Ok(format!("no concept found matching `{id}`"));
                };
                let callers: Vec<Value> = concepts
                    .iter()
                    .filter(|o| o.calls.contains(&c.id))
                    .map(Self::reference)
                    .collect();
                let callees: Vec<Value> = c
                    .calls
                    .iter()
                    .filter_map(|id| find(id))
                    .map(|o| Self::reference(&o))
                    .collect();
                let mut doc = Self::reference(&c);
                doc["schema"] = json!("okf-explore/1");
                doc["callers"] = json!(callers);
                doc["callees"] = json!(callees);
                doc
            }
            ("graph", Some("callers")) => {
                let id = args["id"].as_str().unwrap_or("");
                if find(id).is_none() {
                    return Ok(format!("no concept with id `{id}`"));
                }
                json!({ "schema": "okf-concepts/1", "relation": "callers", "subject": id, "concepts": [] })
            }
            ("graph", Some("file")) => {
                let path = args["path"].as_str().unwrap_or("");
                let found: Vec<Value> = concepts
                    .iter()
                    .filter(|c| c.file == path)
                    .map(Self::reference)
                    .collect();
                json!({ "schema": "okf-concepts/1", "relation": "file", "subject": path, "concepts": found })
            }
            _ => return Ok("unknown".to_string()),
        };
        Ok(doc.to_string())
    }
}

struct Setup {
    _db: Database,
    store: MemoryStore,
    project: ProjectIdentity,
    dir: tempfile::TempDir,
}

/// A project directory with a bundle, and a store using `server`.
async fn setup(server: &Scripted, timeout: Duration) -> Setup {
    let dir = tempfile::tempdir().unwrap();
    let bundle = dir.path().join("knowledge");
    std::fs::create_dir_all(&bundle).unwrap();
    std::fs::write(bundle.join("index.md"), "---\nokf_version: \"0.2\"\n---\n").unwrap();
    let db = Database::connect(&dir.path().join("crustly.db"))
        .await
        .unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default()).with_code_knowledge(
        Detection::Available(CodeKnowledge::new(bundle, timeout, server.connector())),
    );
    let root = std::fs::canonicalize(dir.path()).unwrap();
    let project = ProjectIdentity {
        id: id_for_root(&root),
        root,
        name: "p".into(),
        remote_url: None,
    };
    store.register_project(&project).await.unwrap();
    Setup {
        _db: db,
        store,
        project,
        dir,
    }
}

async fn save(store: &MemoryStore, project: &ProjectIdentity, content: &str) -> Uuid {
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

/// Recall under a small budget, with a long unrelated memory competing,
/// so only relevant memories get in.
async fn recall(s: &Setup, message: &str) -> String {
    s.store
        .recall(&RecallRequest {
            project: &s.project,
            message,
            previous: None,
            window: 1000,
            first_request: false,
            root: Path::new("/nonexistent"),
        })
        .await
        .unwrap_or_default()
}

fn concepts(anchors: Vec<(String, chrono::DateTime<chrono::Utc>)>) -> Vec<String> {
    anchors.into_iter().map(|(c, _)| c).collect()
}

#[tokio::test]
async fn a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not() {
    let server = Scripted::new();
    let s = setup(&server, Duration::from_secs(2)).await;
    assert!(s.store.start_code_knowledge().await);

    let id = save(
        &s.store,
        &s.project,
        "Reuse the shared pool in `connect_pool`.",
    )
    .await;
    assert_eq!(
        concepts(s.store.anchors(id).await.unwrap()),
        ["functions/src/db/pool/connect_pool"]
    );

    let vague = save(
        &s.store,
        &s.project,
        "Always close what `open_handle` returns.",
    )
    .await;
    assert!(
        s.store.anchors(vague).await.unwrap().is_empty(),
        "two concepts are named open_handle"
    );

    let qualified = save(
        &s.store,
        &s.project,
        "Retry `File::open_handle` on Windows sharing errors.",
    )
    .await;
    assert_eq!(
        concepts(s.store.anchors(qualified).await.unwrap()),
        ["functions/src/db/File/open_handle"],
        "the type narrows it to one"
    );
}

#[tokio::test]
async fn naming_the_concept_or_a_caller_recalls_the_anchored_memory() {
    let server = Scripted::new();
    let s = setup(&server, Duration::from_secs(2)).await;
    assert!(s.store.start_code_knowledge().await);
    // No word in common with the requests below: only the anchor links them.
    let id = save(
        &s.store,
        &s.project,
        "One pool per request exhausted the connections.",
    )
    .await;
    s.store
        .set_anchors(id, &["functions/src/db/pool/connect_pool".into()])
        .await
        .unwrap()
        .unwrap();
    save(&s.store, &s.project, &"unrelated words ".repeat(28)).await;

    let direct = recall(&s, "is `connect_pool` thread safe?").await;
    assert!(direct.contains("exhausted the connections"), "{direct}");
    assert!(
        direct.contains("src/db/pool.rs#L10-L20"),
        "the code's place is shown: {direct}"
    );

    let caller = recall(&s, "why does `start_server` hang at boot?").await;
    assert!(
        caller.contains("exhausted the connections"),
        "through a caller: {caller}"
    );

    let other = recall(&s, "how are invoices numbered?").await;
    assert!(!other.contains("exhausted the connections"), "{other}");
}

#[tokio::test]
async fn reading_the_file_that_defines_the_concept_brings_the_memory_once() {
    let server = Scripted::new();
    let s = setup(&server, Duration::from_secs(2)).await;
    assert!(s.store.start_code_knowledge().await);
    let id = save(
        &s.store,
        &s.project,
        "Reuse the shared pool in `connect_pool`.",
    )
    .await;

    let session = Uuid::new_v4();
    let root = s.project.root.clone();
    let found = s
        .store
        .memories_for_path(&s.project, session, "src/db/pool.rs", &root)
        .await
        .unwrap();
    assert_eq!(found.iter().map(|m| m.id).collect::<Vec<_>>(), [id]);
    s.store.mark_shown(session, &[id]);
    assert!(s
        .store
        .memories_for_path(&s.project, session, "src/db/pool.rs", &root)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one() {
    let server = Scripted::new();
    let s = setup(&server, Duration::from_secs(2)).await;
    assert!(s.store.start_code_knowledge().await);
    let id = save(
        &s.store,
        &s.project,
        "Reuse the shared pool in `connect_pool`.",
    )
    .await;
    assert_eq!(s.store.anchors(id).await.unwrap().len(), 1);

    // The concept is gone, but the bundle is the one the anchor was made
    // from: it may just be out of date, so nothing is flagged.
    server.remove("functions/src/db/pool/connect_pool");
    assert!(s.store.drifted(&s.project).await.unwrap().is_empty());

    // A newer bundle without it: flagged, with nothing changed.
    tokio::time::sleep(Duration::from_millis(1100)).await;
    std::fs::write(
        s.dir.path().join("knowledge").join("index.md"),
        "okf_version: \"0.2\"\n",
    )
    .unwrap();
    let drifted = s.store.drifted(&s.project).await.unwrap();
    assert_eq!(drifted.len(), 1);
    assert_eq!(drifted[0].memory.id, id);
    assert_eq!(drifted[0].concept, "functions/src/db/pool/connect_pool");
    assert_eq!(s.store.anchors(id).await.unwrap().len(), 1, "not changed");
    assert!(s.store.get(id).await.unwrap().is_some());
}

#[tokio::test]
async fn an_older_okf_mcp_turns_the_feature_off_and_recall_still_works() {
    let mut server = Scripted::new();
    server.text_only = true;
    let s = setup(&server, Duration::from_secs(2)).await;
    assert!(!s.store.start_code_knowledge().await);
    assert!(s
        .store
        .code_knowledge_status()
        .contains("answers only in text"));

    save(
        &s.store,
        &s.project,
        "Reuse the shared pool in `connect_pool`.",
    )
    .await;
    assert!(recall(&s, "shared pool").await.contains("shared pool"));
}

#[tokio::test]
async fn a_slow_okf_mcp_is_skipped_and_recall_still_works() {
    let mut server = Scripted::new();
    server.delay = Duration::from_millis(300);
    let s = setup(&server, Duration::from_millis(50)).await;
    assert!(
        s.store.start_code_knowledge().await,
        "the start has its own limit"
    );

    let id = save(
        &s.store,
        &s.project,
        "Reuse the shared pool in `connect_pool`.",
    )
    .await;
    assert!(
        s.store.anchors(id).await.unwrap().is_empty(),
        "the lookup timed out"
    );
    assert!(recall(&s, "shared pool").await.contains("shared pool"));
}

#[tokio::test]
async fn without_code_knowledge_nothing_changes() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::connect(&dir.path().join("crustly.db"))
        .await
        .unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    assert_eq!(store.code_knowledge_status(), "off");
    assert!(!store.start_code_knowledge().await);
}

/// Against a real okf-mcp (okf-rs with JSON results): set
/// `CRUSTLY_TEST_OKF_MCP` to its path. Uses a one-concept bundle.
#[tokio::test]
#[ignore = "needs a real okf-mcp; set CRUSTLY_TEST_OKF_MCP"]
async fn real_okf_mcp_answers_in_json() {
    let command = std::env::var("CRUSTLY_TEST_OKF_MCP").expect("CRUSTLY_TEST_OKF_MCP");
    let dir = tempfile::tempdir().unwrap();
    let bundle = dir.path().join("knowledge");
    std::fs::create_dir_all(bundle.join("functions/src/db")).unwrap();
    std::fs::write(bundle.join("index.md"), "---\nokf_version: \"0.2\"\n---\n").unwrap();
    std::fs::write(
        bundle.join("functions/src/db/connect_pool.md"),
        "---\ntype: Rust Function\ntitle: connect_pool\nresource: src/db/pool.rs#L10-L20\n---\n\nbody\n",
    )
    .unwrap();
    let config = crustly::config::MemoryCodeKnowledgeConfig {
        command,
        query_timeout_ms: 2000,
        ..Default::default()
    };
    let Detection::Available(code) = CodeKnowledge::detect(&config, dir.path()) else {
        panic!("detected off");
    };
    assert!(code.start().await, "{}", code.status_line());
    let ident = crustly::memory::code_knowledge::identifiers("`connect_pool`");
    assert_eq!(
        code.resolve(&ident[0]).await.as_deref(),
        Some("functions/src/db/connect_pool")
    );
    assert_eq!(
        code.location("functions/src/db/connect_pool")
            .await
            .as_deref(),
        Some("src/db/pool.rs#L10-L20")
    );
    assert_eq!(code.file_concepts("src/db/pool.rs").await.len(), 1);
    assert_eq!(code.exists("functions/src/db/nope").await, Some(false));
    let _ = PathBuf::new();
}
