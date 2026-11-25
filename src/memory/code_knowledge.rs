//! Code knowledge from an okf-rs bundle, through okf-mcp
//! (docs/improve-memory.md §23; tasks P6.4 to P6.7).
//!
//! Memories hold what the code cannot tell; an okf-rs bundle holds what it
//! does: functions, types, and who calls what. This module connects the
//! two. It finds the bundle and the `okf-mcp` command, starts the server
//! (in the background for a chat, awaited by a CLI command), checks that it
//! answers in JSON, and offers the few queries memory needs: which concept
//! a name in a memory or a message refers to, its callers and callees, the
//! concepts of a file, where a concept is defined, and whether it still
//! exists.
//!
//! Nothing here is required. A missing command or bundle turns the feature
//! off with a reason for `crustly memory status`; a slow or failed query
//! returns nothing, and recall carries on without code knowledge.

use crate::config::MemoryCodeKnowledgeConfig;
use crate::mcp::client::{MCPClient, McpToolDef};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Most anchors one memory gets (§23.3).
pub const MAX_ANCHORS: usize = 3;

/// Most names looked up for one text: each is one query.
const MAX_LOOKUPS: usize = 6;

/// Concepts whose callers and callees are followed during recall (§23.3).
pub const MAX_NEIGHBOUR_SOURCES: usize = 3;

/// Longest wait for okf-mcp to start and load the bundle.
const START_TIMEOUT: Duration = Duration::from_secs(30);

/// `search_ranked` returns at most this many hits; when all of them carry
/// the searched name, more may exist, so the name is ambiguous.
const SEARCH_HIT_CAP: usize = 25;

const SEARCH_SCHEMA: &str = "okf-search-ranked/1";
const EXPLORE_SCHEMA: &str = "okf-explore/1";
const CONCEPTS_SCHEMA: &str = "okf-concepts/1";

/// Names too common to point at one concept, even when a bundle happens to
/// have only one of them.
const COMMON_NAMES: &[&str] = &[
    "new", "get", "set", "run", "main", "test", "tests", "from", "into", "default", "build",
    "clone", "fmt", "len", "iter", "map", "drop", "init", "self", "Self", "String", "Vec",
    "Option", "Result", "Box", "Arc", "Mutex", "None", "Some", "Ok", "Err", "true", "false",
];

// ── Names in text ────────────────────────────────────────────────────────────

/// A name in a memory or a message that may refer to a code concept: a
/// function, method or type name, with the type or module it was written
/// under (`Database::connect` gives `connect` under `Database`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
    pub qualifier: Option<String>,
}

/// The names in `text` worth looking up, in order, without repeats:
/// backticked words, `snake_case` and `CamelCase` tokens, and
/// `Type::name` paths. Commands and file paths in backticks are left out
/// (paths are matched by path-scoped memories instead), and so are names
/// too short or too common to point at one concept.
pub fn identifiers(text: &str) -> Vec<Identifier> {
    let mut found: Vec<Identifier> = Vec::new();
    let mut push = |ident: Option<Identifier>| {
        if let Some(ident) = ident {
            if !found.contains(&ident) {
                found.push(ident);
            }
        }
    };
    for (i, part) in text.split('`').enumerate() {
        if i % 2 == 1 {
            push(parse_code_span(part));
        } else {
            for token in part.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == ':'))
            {
                let token = token.trim_matches(':');
                if token.contains("::") || is_snake_case(token) || is_camel_case(token) {
                    push(parse_code_span(token));
                }
            }
        }
    }
    found
}

/// The identifier a backticked span names: `foo()`, `Type::foo`,
/// `self.foo`, `&Type`. `None` for commands, paths and plain words.
fn parse_code_span(span: &str) -> Option<Identifier> {
    let span = span.trim();
    if span.is_empty() || span.contains(char::is_whitespace) || span.contains('/') {
        return None;
    }
    let span = span.split('(').next().unwrap_or(span);
    let span = span
        .trim_start_matches(['&', '*'])
        .trim_end_matches(['!', '?', ';', ',']);
    let segments: Vec<&str> = span
        .split("::")
        .flat_map(|s| s.split('.'))
        .filter(|s| !s.is_empty())
        .collect();
    let name = (*segments.last()?).to_string();
    if !is_identifier(&name) || name.len() < 3 || COMMON_NAMES.contains(&name.as_str()) {
        return None;
    }
    let qualifier = segments
        .len()
        .checked_sub(2)
        .map(|i| segments[i])
        .filter(|q| is_identifier(q) && *q != "self" && *q != "Self" && *q != "crate")
        .map(str::to_string);
    // A plain lower-case word (`open`, `recall`) is too often an ordinary
    // word, or one of many functions, to name one concept on its own: it
    // needs its type (`File::open`). Compound names and type names do not.
    let distinctive = is_snake_case(&name)
        || is_camel_case(&name)
        || name.chars().next().is_some_and(|c| c.is_ascii_uppercase());
    if qualifier.is_none() && !distinctive {
        return None;
    }
    Some(Identifier { name, qualifier })
}

/// Whether a concept id is test code: a `tests`/`test` path segment, or a
/// `test_*`/`*_test(s)` name. Memories are about the code under test, so
/// test concepts are never anchor targets.
fn is_test_concept(id: &str) -> bool {
    id.split('/').any(|part| {
        part == "tests"
            || part == "test"
            || part.starts_with("test_")
            || part.ends_with("_test")
            || part.ends_with("_tests")
    })
}

fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// `connect_pool`: lower-case words joined by underscores.
fn is_snake_case(s: &str) -> bool {
    s.len() >= 3
        && s.contains('_')
        && !s.starts_with('_')
        && !s.ends_with('_')
        && s.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// `MemoryStore`: an upper-case start, lower-case letters, and another
/// upper-case letter later.
fn is_camel_case(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && s.chars().all(|c| c.is_ascii_alphanumeric())
        && s.chars().any(|c| c.is_ascii_lowercase())
        && s.chars().skip(1).any(|c| c.is_ascii_uppercase())
}

// ── The server ───────────────────────────────────────────────────────────────

/// An okf-mcp server, as this module talks to it. The real one is an MCP
/// client over stdio; tests use a scripted one.
#[async_trait]
pub trait OkfServer: Send {
    async fn tools(&mut self) -> Result<Vec<McpToolDef>>;
    async fn call(&mut self, tool: &str, arguments: Value) -> Result<String>;
}

#[async_trait]
impl OkfServer for MCPClient {
    async fn tools(&mut self) -> Result<Vec<McpToolDef>> {
        self.discover_tools().await
    }

    async fn call(&mut self, tool: &str, arguments: Value) -> Result<String> {
        self.call_tool(tool, arguments).await
    }
}

/// Starts a server.
pub type Connector = Box<dyn Fn() -> BoxFuture<'static, Result<Box<dyn OkfServer>>> + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Starting,
    Ready,
    Off(String),
}

/// What detection found: the reason code knowledge is off, or a handle.
pub enum Detection {
    Off(String),
    Available(CodeKnowledge),
}

/// The connection to one project's okf-mcp, with the queries memory uses.
pub struct CodeKnowledge {
    connector: Connector,
    server: tokio::sync::Mutex<Option<Box<dyn OkfServer>>>,
    status: Mutex<Status>,
    timeout: Duration,
    bundle: PathBuf,
}

/// The bundle of the project at `root`: `okf.toml`'s `output`, or
/// `knowledge`, when its `index.md` declares an `okf_version`.
pub fn bundle_dir(root: &Path) -> Option<PathBuf> {
    let output = std::fs::read_to_string(root.join("okf.toml"))
        .ok()
        .and_then(|text| text.parse::<toml::Table>().ok())
        .and_then(|table| table.get("output")?.as_str().map(str::to_string))
        .unwrap_or_else(|| "knowledge".to_string());
    let bundle = root.join(output);
    let index = std::fs::read_to_string(bundle.join("index.md")).ok()?;
    index.contains("okf_version").then_some(bundle)
}

impl CodeKnowledge {
    /// Whether code knowledge can be used for the project at `root`, by the
    /// settings: off, no bundle, or no command make it off, with a reason.
    pub fn detect(config: &MemoryCodeKnowledgeConfig, root: &Path) -> Detection {
        if config.enabled == "off" {
            return Detection::Off("off ([memory.code_knowledge] enabled = \"off\")".to_string());
        }
        let off = |reason: String| {
            if config.enabled == "on" {
                tracing::warn!("code knowledge is off: {reason}");
            }
            Detection::Off(reason)
        };
        let Some(bundle) = bundle_dir(root) else {
            return off(
                "no okf-rs bundle in this project (`okf-rs generate` makes one)".to_string(),
            );
        };
        let Ok(command) = which::which(&config.command) else {
            return off(format!(
                "`{}` was not found (install okf-rs, or set [memory.code_knowledge] command)",
                config.command
            ));
        };
        let root = root.to_path_buf();
        let connector: Connector = Box::new(move || {
            let command = command.clone();
            let root = root.clone();
            Box::pin(async move {
                let root = root.to_string_lossy().into_owned();
                let client =
                    MCPClient::connect("okf", &command.to_string_lossy(), &[root.as_str()]).await?;
                Ok(Box::new(client) as Box<dyn OkfServer>)
            })
        });
        Detection::Available(Self::new(
            bundle,
            Duration::from_millis(config.query_timeout_ms),
            connector,
        ))
    }

    /// A handle that starts its server with `connector` (tests pass a
    /// scripted one).
    pub fn new(bundle: PathBuf, timeout: Duration, connector: Connector) -> Self {
        Self {
            connector,
            server: tokio::sync::Mutex::new(None),
            status: Mutex::new(Status::Idle),
            timeout,
            bundle,
        }
    }

    fn status(&self) -> Status {
        self.status
            .lock()
            .map(|s| s.clone())
            .unwrap_or_else(|_| Status::Off("internal error".to_string()))
    }

    fn set_status(&self, status: Status) {
        if let Ok(mut current) = self.status.lock() {
            *current = status;
        }
    }

    /// Whether queries can be answered now.
    pub fn is_ready(&self) -> bool {
        self.status() == Status::Ready
    }

    /// One line for `crustly memory status`.
    pub fn status_line(&self) -> String {
        match self.status() {
            Status::Idle => format!("okf-mcp, bundle {} (not started)", self.bundle_shown()),
            Status::Starting => format!("okf-mcp, bundle {} (starting)", self.bundle_shown()),
            Status::Ready => format!("okf-mcp, bundle {}", self.bundle_shown()),
            Status::Off(reason) => format!("off: {reason}"),
        }
    }

    /// The bundle path for people: without Windows's `\\?\` prefix.
    fn bundle_shown(&self) -> String {
        let text = self.bundle.to_string_lossy();
        text.strip_prefix(r"\\?\").unwrap_or(&text).to_string()
    }

    /// The bundle directory.
    pub fn bundle(&self) -> &Path {
        &self.bundle
    }

    /// Start the server, check it answers in JSON, and load the bundle.
    /// Returns whether it is ready. Concurrent calls wait for one start;
    /// a failed start turns the feature off for the rest of the process.
    pub async fn start(&self) -> bool {
        let mut guard = self.server.lock().await;
        match self.status() {
            Status::Ready => return true,
            Status::Off(_) => return false,
            Status::Idle | Status::Starting => {}
        }
        self.set_status(Status::Starting);
        let started = tokio::time::timeout(START_TIMEOUT, async {
            let mut server = (self.connector)().await?;
            check_capabilities(&server.tools().await?)?;
            // Loads the bundle and builds the search index, so the first
            // real query is answered in milliseconds.
            server
                .call(
                    "search_ranked",
                    json!({ "query": "warm up", "format": "json" }),
                )
                .await?;
            Ok::<_, anyhow::Error>(server)
        })
        .await;
        match started {
            Ok(Ok(server)) => {
                *guard = Some(server);
                self.set_status(Status::Ready);
                true
            }
            Ok(Err(err)) => {
                tracing::info!("code knowledge is off: {err:#}");
                self.set_status(Status::Off(format!("{err:#}")));
                false
            }
            Err(_) => {
                self.set_status(Status::Off(format!(
                    "okf-mcp did not start within {} s",
                    START_TIMEOUT.as_secs()
                )));
                false
            }
        }
    }

    /// Start the server in the background, once; queries before it is
    /// ready return nothing.
    pub fn spawn_start(self: &Arc<Self>) {
        if self.status() == Status::Idle {
            let me = self.clone();
            tokio::spawn(async move {
                me.start().await;
            });
        }
    }

    /// Call `tool` and read a JSON answer with `schema`. `None` when the
    /// server is not ready, too slow, or failed; `Some(Err(text))` when it
    /// answered with text instead (a tool error such as an unknown id).
    async fn call_json(
        &self,
        tool: &str,
        arguments: Value,
        schema: &str,
    ) -> Option<std::result::Result<Value, String>> {
        if self.status() != Status::Ready {
            return None;
        }
        let call = async {
            let mut guard = self.server.lock().await;
            let server = guard.as_mut()?;
            Some(server.call(tool, arguments).await)
        };
        match tokio::time::timeout(self.timeout, call).await {
            Err(_) => {
                tracing::debug!("okf-mcp {tool} took longer than {:?}", self.timeout);
                None
            }
            Ok(None) => None,
            Ok(Some(Err(err))) => {
                tracing::info!("code knowledge is off: okf-mcp failed: {err:#}");
                self.set_status(Status::Off(format!("okf-mcp stopped: {err:#}")));
                *self.server.lock().await = None;
                None
            }
            Ok(Some(Ok(text))) => match serde_json::from_str::<Value>(&text) {
                Ok(doc) if doc["schema"] == schema => Some(Ok(doc)),
                Ok(doc) => {
                    tracing::debug!("okf-mcp answered an unknown schema: {}", doc["schema"]);
                    None
                }
                Err(_) => Some(Err(text)),
            },
        }
    }

    /// The one concept `ident` names: an exact title match, narrowed by its
    /// qualifier, and unambiguous. `None` when there is no match, more than
    /// one, or no answer.
    pub async fn resolve(&self, ident: &Identifier) -> Option<String> {
        let doc = self
            .call_json(
                "search_ranked",
                json!({ "query": ident.name, "format": "json" }),
                SEARCH_SCHEMA,
            )
            .await?
            .ok()?;
        let hits = doc["hits"].as_array()?;
        let exact: Vec<&Value> = hits
            .iter()
            .filter(|h| h["title"].as_str() == Some(ident.name.as_str()))
            .collect();
        if hits.len() >= SEARCH_HIT_CAP && exact.len() == hits.len() {
            // Every hit is a match and the list was cut: there may be more.
            return None;
        }
        let mut ids: Vec<String> = exact
            .iter()
            .filter_map(|h| h["id"].as_str())
            .filter(|id| !is_test_concept(id))
            .filter(|id| {
                ident
                    .qualifier
                    .as_deref()
                    .is_none_or(|q| id.split('/').any(|part| part == q))
            })
            .map(str::to_string)
            .collect();
        ids.sort();
        ids.dedup();
        (ids.len() == 1).then(|| ids.remove(0))
    }

    /// The concepts the names in `text` refer to, at most `MAX_ANCHORS`,
    /// in order. Ambiguous and unknown names are skipped.
    pub async fn resolve_text(&self, text: &str) -> Vec<String> {
        if !self.is_ready() {
            return Vec::new();
        }
        let mut concepts = Vec::new();
        for ident in identifiers(text).iter().take(MAX_LOOKUPS) {
            if let Some(id) = self.resolve(ident).await {
                if !concepts.contains(&id) {
                    concepts.push(id);
                }
                if concepts.len() >= MAX_ANCHORS {
                    break;
                }
            }
        }
        concepts
    }

    /// The direct callers and callees of `id` (at most 20 of each).
    pub async fn neighbours(&self, id: &str) -> Vec<String> {
        let Some(Ok(doc)) = self.explore(id).await else {
            return Vec::new();
        };
        ["callers", "callees"]
            .iter()
            .filter_map(|list| doc[*list].as_array())
            .flatten()
            .filter_map(|c| c["id"].as_str().map(str::to_string))
            .collect()
    }

    /// `explore` on an exact id; an answer about another concept (explore
    /// falls back to a search) counts as no answer.
    async fn explore(&self, id: &str) -> Option<std::result::Result<Value, String>> {
        match self
            .call_json(
                "explore",
                json!({ "query": id, "format": "json" }),
                EXPLORE_SCHEMA,
            )
            .await?
        {
            Ok(doc) if doc["id"] == id => Some(Ok(doc)),
            Ok(_) => Some(Err(format!("no concept with id `{id}`"))),
            Err(text) => Some(Err(text)),
        }
    }

    /// The concepts defined in `path`, relative to the project root.
    pub async fn file_concepts(&self, path: &str) -> Vec<String> {
        let Some(Ok(doc)) = self
            .call_json(
                "graph",
                json!({ "relation": "file", "path": path, "format": "json" }),
                CONCEPTS_SCHEMA,
            )
            .await
        else {
            return Vec::new();
        };
        doc["concepts"]
            .as_array()
            .map(|list| {
                list.iter()
                    .filter_map(|c| c["id"].as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Where `id` is defined now: `file#Lstart-Lend`.
    pub async fn location(&self, id: &str) -> Option<String> {
        let doc = self.explore(id).await?.ok()?;
        let location = &doc["location"];
        let file = location["file"].as_str()?;
        let start = location["start_line"].as_u64()?;
        let end = location["end_line"].as_u64()?;
        Some(if start == end {
            format!("{file}#L{start}")
        } else {
            format!("{file}#L{start}-L{end}")
        })
    }

    /// Whether `id` is in the bundle: `Some(false)` only when okf-mcp says
    /// it has no such concept; `None` when it cannot tell.
    pub async fn exists(&self, id: &str) -> Option<bool> {
        match self
            .call_json(
                "graph",
                json!({ "relation": "callers", "id": id, "format": "json" }),
                CONCEPTS_SCHEMA,
            )
            .await?
        {
            Ok(_) => Some(true),
            Err(text) if text.contains("no concept with id") => Some(false),
            Err(_) => None,
        }
    }

    /// The concept closest to a missing `id`, by its name: the best search
    /// hit, for re-anchoring a memory whose code was renamed.
    pub async fn closest(&self, id: &str) -> Option<String> {
        let name = id.rsplit('/').next()?;
        let doc = self
            .call_json(
                "search_ranked",
                json!({ "query": name, "format": "json" }),
                SEARCH_SCHEMA,
            )
            .await?
            .ok()?;
        doc["hits"]
            .as_array()?
            .first()?
            .get("id")?
            .as_str()
            .map(str::to_string)
    }

    /// When the bundle last changed: the newest modification time of its
    /// files. Anchors record it, so drift is only judged against a newer
    /// bundle (§23.4).
    pub fn bundle_time(&self) -> Option<DateTime<Utc>> {
        newest_mtime(&self.bundle).map(DateTime::<Utc>::from)
    }

    /// Whether a source file under `root` changed after the bundle, so
    /// `okf-rs generate` would update it (§23.2). Files git ignores, and the
    /// bundle itself, are left out. Crustly never regenerates the bundle: it
    /// writes into the working tree.
    pub fn bundle_is_older_than_sources(&self, root: &Path) -> bool {
        const SOURCE: &[&str] = &[
            "rs", "py", "js", "jsx", "ts", "tsx", "go", "java", "kt", "kts", "c", "h", "cc", "cpp",
            "hpp", "cs", "rb", "php", "swift", "scala",
        ];
        let Some(bundle_time) = newest_mtime(&self.bundle) else {
            return false;
        };
        let bundle = std::fs::canonicalize(&self.bundle).unwrap_or_else(|_| self.bundle.clone());
        ignore::WalkBuilder::new(root)
            .build()
            .flatten()
            .filter(|entry| {
                let path = entry.path();
                !path.starts_with(&bundle) && !path.starts_with(&self.bundle)
            })
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| SOURCE.contains(&e))
            })
            .any(|entry| {
                entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .is_some_and(|modified| modified > bundle_time)
            })
    }
}

/// The newest modification time of the files under `dir`.
fn newest_mtime(dir: &Path) -> Option<std::time::SystemTime> {
    let mut newest = None;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else {
                continue;
            };
            if meta.is_dir() {
                stack.push(entry.path());
            } else if let Ok(modified) = meta.modified() {
                newest = Some(newest.map_or(modified, |n: std::time::SystemTime| n.max(modified)));
            }
        }
    }
    newest
}

/// Whether okf-mcp has what this module needs: JSON results (`format` on
/// `search_ranked`, `explore` and `graph`) and the `file` relation. Older
/// servers answer only in text, which is not parsed (§23.5).
fn check_capabilities(tools: &[McpToolDef]) -> Result<()> {
    let schema = |name: &str| {
        tools
            .iter()
            .find(|t| t.name == name)
            .and_then(|t| t.input_schema.clone())
    };
    let has_json = |name: &str| {
        schema(name).is_some_and(|s| {
            s["properties"]["format"]["enum"]
                .as_array()
                .is_some_and(|values| values.contains(&json!("json")))
        })
    };
    let has_file = schema("graph").is_some_and(|s| {
        s["properties"]["relation"]["enum"]
            .as_array()
            .is_some_and(|values| values.contains(&json!("file")))
    });
    if ["search_ranked", "explore", "graph"]
        .iter()
        .all(|t| has_json(t))
        && has_file
    {
        Ok(())
    } else {
        Err(anyhow!(
            "this okf-mcp answers only in text; okf-rs with JSON results and the `file` relation is needed"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(text: &str) -> Vec<(String, Option<String>)> {
        identifiers(text)
            .into_iter()
            .map(|i| (i.name, i.qualifier))
            .collect()
    }

    #[test]
    fn backticked_snake_and_camel_names_are_found() {
        assert_eq!(
            names(
                "Reuse the shared pool in `connect_pool`; MemoryStore owns it, see record_recall."
            ),
            vec![
                ("connect_pool".to_string(), None),
                ("MemoryStore".to_string(), None),
                ("record_recall".to_string(), None),
            ]
        );
    }

    #[test]
    fn a_qualified_name_keeps_its_type() {
        assert_eq!(
            names("Call `Database::run_migrations()` before `self.recall`, and Store::save too."),
            vec![
                ("run_migrations".to_string(), Some("Database".to_string())),
                ("save".to_string(), Some("Store".to_string())),
            ]
        );
    }

    /// A plain word needs its type: `open` alone is an ordinary word, or one
    /// of many functions; `File::open` is one. Type names stand alone.
    #[test]
    fn a_plain_word_needs_its_type_and_a_type_name_does_not() {
        assert!(names("Always close what `open` returns, see `self.recall`.").is_empty());
        assert_eq!(
            names("`File::open` and `Database` and open_pool"),
            vec![
                ("open".to_string(), Some("File".to_string())),
                ("Database".to_string(), None),
                ("open_pool".to_string(), None),
            ]
        );
    }

    #[test]
    fn test_code_is_recognised_by_its_path_or_name() {
        assert!(is_test_concept(
            "functions/tests/memory_acceptance_test/open"
        ));
        assert!(is_test_concept(
            "functions/src/memory/recall/tests/select_ranks"
        ));
        assert!(is_test_concept("functions/src/db/test_connect"));
        assert!(!is_test_concept("functions/src/db/Database/run_migrations"));
        assert!(!is_test_concept("functions/src/contest/attest"));
    }

    #[test]
    fn commands_paths_and_common_words_are_left_out() {
        assert!(names("Run `cargo test`, edit `src/db/mod.rs`, call `new()` or `get`.").is_empty());
        assert!(names("Uses Redis for caching. The API is fast.").is_empty());
        assert_eq!(
            names("`foo_bar` and `foo_bar` again"),
            vec![("foo_bar".to_string(), None)]
        );
    }

    #[test]
    fn capabilities_need_json_and_the_file_relation() {
        let tool = |name: &str, schema: Value| McpToolDef {
            name: name.to_string(),
            description: None,
            input_schema: Some(schema),
        };
        let format = json!({ "properties": { "format": { "enum": ["text", "json"] } } });
        let graph = json!({ "properties": {
            "format": { "enum": ["text", "json"] },
            "relation": { "enum": ["callers", "file"] }
        } });
        let new = vec![
            tool("search_ranked", format.clone()),
            tool("explore", format.clone()),
            tool("graph", graph),
        ];
        assert!(check_capabilities(&new).is_ok());
        let old = vec![
            tool("search_ranked", json!({ "properties": {} })),
            tool("explore", json!({ "properties": {} })),
            tool(
                "graph",
                json!({ "properties": { "relation": { "enum": ["callers"] } } }),
            ),
        ];
        assert!(check_capabilities(&old).is_err());
    }

    #[test]
    fn a_bundle_is_found_through_okf_toml_or_knowledge() {
        let dir = tempfile::tempdir().unwrap();
        assert!(bundle_dir(dir.path()).is_none());
        std::fs::create_dir_all(dir.path().join("knowledge")).unwrap();
        std::fs::write(
            dir.path().join("knowledge").join("index.md"),
            "---\nokf_version: \"0.2\"\n---\n",
        )
        .unwrap();
        assert_eq!(bundle_dir(dir.path()), Some(dir.path().join("knowledge")));

        std::fs::write(dir.path().join("okf.toml"), "output = \"kb\"\n").unwrap();
        assert!(
            bundle_dir(dir.path()).is_none(),
            "okf.toml names another bundle"
        );
        std::fs::create_dir_all(dir.path().join("kb")).unwrap();
        std::fs::write(
            dir.path().join("kb").join("index.md"),
            "okf_version: \"0.2\"",
        )
        .unwrap();
        assert_eq!(bundle_dir(dir.path()), Some(dir.path().join("kb")));
    }

    #[test]
    fn detection_is_off_without_a_command_or_a_bundle() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = MemoryCodeKnowledgeConfig::default();
        assert!(matches!(
            CodeKnowledge::detect(&config, dir.path()),
            Detection::Off(reason) if reason.contains("no okf-rs bundle")
        ));
        std::fs::create_dir_all(dir.path().join("knowledge")).unwrap();
        std::fs::write(
            dir.path().join("knowledge").join("index.md"),
            "okf_version: x",
        )
        .unwrap();
        config.command = "definitely-not-okf-mcp".to_string();
        assert!(matches!(
            CodeKnowledge::detect(&config, dir.path()),
            Detection::Off(reason) if reason.contains("was not found")
        ));
        config.enabled = "off".to_string();
        assert!(matches!(
            CodeKnowledge::detect(&config, dir.path()),
            Detection::Off(reason) if reason.starts_with("off")
        ));
    }
}
