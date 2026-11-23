//! Save Memory Tool (qwen-code/Gemini CLI compatible)
//!
//! qwen-code's `save_memory` tool (inherited from Gemini CLI, whose fork it
//! is - `memory-config.ts` re-exports `getCurrentGeminiMdFilename` and
//! friends, confirming the shared lineage) lets the model persist a fact
//! *across sessions*: `{"fact": "<self-contained statement>"}`, no other
//! fields.
//!
//! This is deliberately a real tool, not an alias to `session_context`'s
//! `add_fact` operation: that operation's store is keyed by
//! `context_{session_id}.json`, so it disappears once the session ends.
//! Pointing `save_memory` at it would make the call succeed while silently
//! breaking the one thing a model relying on "remembered" actually needs -
//! that the fact outlives this conversation. This tool instead appends to a
//! file keyed by working directory (`.crustly/memory.md`), so it persists
//! for every future session in the same project.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use crate::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use crate::memory::MemoryStore;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;

/// Saves a fact for later sessions. With a memory store it writes to the
/// database (docs/improve-memory.md §10.1); without one it appends to
/// `.crustly/memory.md`, as in v1.
pub struct SaveMemoryTool {
    store: Option<Arc<MemoryStore>>,
}

impl SaveMemoryTool {
    /// The tool backed by long-term memory when `store` is set.
    pub fn new(store: Option<Arc<MemoryStore>>) -> Self {
        Self { store }
    }

    /// The v1 tool, writing `.crustly/memory.md` (used when long-term memory
    /// is off, and in tests of the file path).
    pub fn file_only() -> Self {
        Self { store: None }
    }

    /// Save into the database, returning what the model should be told.
    async fn save_to_store(
        &self,
        store: &MemoryStore,
        input: SaveMemoryInput,
        context: &ToolExecutionContext,
    ) -> Result<ToolResult> {
        let fact = input.fact.trim().to_string();
        let project = crate::memory::project::identify(&context.working_directory);
        if let Err(err) = store.register_project(&project).await {
            return Ok(ToolResult::error(format!(
                "Could not record the project: {err}"
            )));
        }

        let global = input.scope.as_deref() == Some("global");
        let scope = if global {
            MemoryScope::Global
        } else {
            MemoryScope::Project(project.id.clone())
        };

        let default_category = if global { "preference" } else { "fact" };
        let category = match input
            .category
            .as_deref()
            .unwrap_or(default_category)
            .parse()
        {
            Ok(MemoryCategory::Episode) | Err(_) => {
                return Ok(ToolResult::error(
                    "category must be one of: preference, convention, decision, fact, lesson"
                        .to_string(),
                ))
            }
            Ok(category) => category,
        };

        let replaces = match input.replaces.as_deref() {
            None => None,
            Some(prefix) => match store.find(prefix).await {
                Ok(matches) if matches.len() == 1 => Some(matches[0].id),
                Ok(matches) if matches.is_empty() => {
                    return Ok(ToolResult::error(format!("No memory with id {prefix}")))
                }
                Ok(_) => {
                    return Ok(ToolResult::error(format!(
                        "The id {prefix} matches more than one memory; use a longer prefix"
                    )))
                }
                Err(err) => return Ok(ToolResult::error(format!("Lookup failed: {err}"))),
            },
        };

        let expires_at = match input.expires_in_days {
            None => None,
            Some(days) if (1..=365).contains(&days) => {
                Some(chrono::Utc::now() + chrono::Duration::days(days))
            }
            Some(_) => {
                return Ok(ToolResult::error(
                    "expires_in_days must be between 1 and 365".to_string(),
                ))
            }
        };

        let outcome = store
            .save(NewMemory {
                content: fact.clone(),
                scope,
                category,
                source: MemorySource::Tool,
                importance: 3,
                replaces,
                session_id: Some(context.session_id),
                expires_at,
            })
            .await;
        // Paths attach to a saved memory, not to the save itself, so a bad
        // path is reported without losing the fact.
        let mut path_note = String::new();
        if let (Ok(SaveOutcome::Created(id)), Some(paths)) = (&outcome, &input.paths) {
            if !paths.is_empty() {
                if let Err(reason) = store
                    .add_paths(*id, paths)
                    .await
                    .unwrap_or_else(|e| Err(e.to_string()))
                {
                    path_note = format!(" (paths not added: {reason})");
                }
            }
        }
        Ok(match outcome {
            Ok(SaveOutcome::Created(id)) => ToolResult::success(format!(
                "Saved{} [m:{}]: {fact}{path_note}",
                if global { " to global memory" } else { "" },
                &id.simple().to_string()[..8]
            )),
            Ok(SaveOutcome::Duplicate(id)) => ToolResult::success(format!(
                "Already remembered [m:{}]",
                &id.simple().to_string()[..8]
            )),
            Ok(SaveOutcome::Rejected(reason)) => ToolResult::error(reason.to_string()),
            Err(err) => ToolResult::error(format!("Could not save the memory: {err}")),
        })
    }
}

#[derive(Debug, Deserialize)]
struct SaveMemoryInput {
    fact: String,
    /// "project" (default) or "global".
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    category: Option<String>,
    /// Id, or unique id prefix, of a recalled memory this one corrects.
    #[serde(default)]
    replaces: Option<String>,
    /// For facts that stop being true at a known time.
    #[serde(default)]
    expires_in_days: Option<i64>,
    /// Project paths (globs) this fact is about, up to five.
    #[serde(default)]
    paths: Option<Vec<String>>,
}

const MEMORY_FILE: &str = "memory.md";
const MEMORY_HEADER: &str = "## Memories";

fn memory_path(working_directory: &Path) -> PathBuf {
    working_directory.join(".crustly").join(MEMORY_FILE)
}

/// Append `fact` as a new bullet under `MEMORY_HEADER`, adding the header if
/// it's missing (empty file, or a file that predates this tool). A no-op
/// (returns `existing` unchanged, with `false`) if `fact` is already present
/// verbatim, so re-remembering the same thing doesn't pile up duplicates.
fn append_fact(existing: &str, fact: &str) -> (String, bool) {
    let bullet = format!("- {fact}");

    let mut lines: Vec<String> = if existing.trim().is_empty() {
        vec![MEMORY_HEADER.to_string()]
    } else if existing.lines().next().map(str::trim) == Some(MEMORY_HEADER) {
        existing.lines().map(String::from).collect()
    } else {
        let mut v = vec![MEMORY_HEADER.to_string()];
        v.extend(existing.lines().map(String::from));
        v
    };

    if lines.iter().any(|l| l.trim() == bullet) {
        return (existing.to_string(), false);
    }

    lines.push(bullet);
    let mut result = lines.join("\n");
    result.push('\n');
    (result, true)
}

#[async_trait]
impl Tool for SaveMemoryTool {
    fn name(&self) -> &str {
        "save_memory"
    }

    fn description(&self) -> &str {
        "Save a specific, self-contained fact to long-term project memory, so it's available in \
         future sessions in this project (not just the rest of this conversation). Use for \
         durable facts worth remembering across sessions - user preferences, project \
         conventions, standing decisions. For task-scoped state that shouldn't outlive this \
         conversation, use todo_write or session_context instead."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "fact": {
                    "type": "string",
                    "description": "The specific fact to remember. Should be a clear, self-contained statement (e.g. 'The user prefers tabs over spaces', not 'use tabs')."
                },
                "scope": {
                    "type": "string",
                    "enum": ["project", "global"],
                    "description": "project (default): true of this repository only. global: true of the user in every project, such as a personal preference. Global saves may need the user's approval."
                },
                "category": {
                    "type": "string",
                    "enum": ["preference", "convention", "decision", "fact", "lesson"]
                },
                "replaces": {
                    "type": "string",
                    "description": "Id of a recalled memory that this fact corrects or updates. The old memory is kept as history."
                },
                "paths": {
                    "type": "array",
                    "items": { "type": "string" },
                    "maxItems": 5,
                    "description": "Files or folders this fact is about, relative to the project root, e.g. src/tui/**. Recalled when a request or a tool call mentions them."
                },
                "expires_in_days": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 365,
                    "description": "Only for facts that stop being true at a known time, e.g. 'use the release branch until Friday'."
                }
            },
            "required": ["fact"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles]
    }

    fn requires_approval(&self) -> bool {
        false // Remembering a fact is low-risk, same as todo_write/session_context.
    }

    /// A global save follows the user into every project, so it needs the
    /// user's approval. Project saves stay on the same footing as the v1 file:
    /// no approval. Without a store, the scope is ignored, so nothing needs it.
    fn requires_approval_for(&self, input: &Value) -> bool {
        self.store.is_some() && input.get("scope").and_then(Value::as_str) == Some("global")
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: SaveMemoryInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        if input.fact.trim().is_empty() {
            return Err(ToolError::InvalidInput("fact cannot be empty".to_string()));
        }
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        // Matches todo_write's precedent: bookkeeping writes under
        // .crustly/ are still blocked in Plan mode, since Plan mode's
        // guarantee is "nothing changes until the plan is approved."
        if context.read_only_mode {
            return Err(ToolError::PermissionDenied(
                "Cannot save memory in read-only (plan) mode".to_string(),
            ));
        }

        let input: SaveMemoryInput = serde_json::from_value(input)?;
        if let Some(store) = self.store.clone() {
            return self.save_to_store(&store, input, context).await;
        }
        let fact = input.fact.trim();
        if fact.is_empty() {
            return Ok(ToolResult::error("fact cannot be empty".to_string()));
        }

        let path = memory_path(&context.working_directory);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(ToolError::Io)?;
        }

        let existing = if path.exists() {
            fs::read_to_string(&path).await.map_err(ToolError::Io)?
        } else {
            String::new()
        };

        let (new_content, added) = append_fact(&existing, fact);
        if added {
            fs::write(&path, &new_content)
                .await
                .map_err(ToolError::Io)?;
        }

        let message = if added {
            format!("Remembered: {fact}")
        } else {
            format!("Already remembered: {fact}")
        };

        Ok(ToolResult::success(message)
            .with_metadata("path".to_string(), path.display().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn context(temp_dir: &TempDir) -> ToolExecutionContext {
        ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf())
    }

    #[tokio::test]
    async fn execute_creates_memory_file_with_header_and_fact() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool::file_only();

        let result = tool
            .execute(
                serde_json::json!({ "fact": "The user prefers tabs over spaces" }),
                &context(&temp_dir),
            )
            .await
            .unwrap();

        assert!(result.success, "{:?}", result.error);
        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert_eq!(
            contents,
            "## Memories\n- The user prefers tabs over spaces\n"
        );
    }

    #[tokio::test]
    async fn execute_appends_to_existing_memory_file() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool::file_only();
        let ctx = context(&temp_dir);

        tool.execute(serde_json::json!({ "fact": "first fact" }), &ctx)
            .await
            .unwrap();
        tool.execute(serde_json::json!({ "fact": "second fact" }), &ctx)
            .await
            .unwrap();

        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert_eq!(contents, "## Memories\n- first fact\n- second fact\n");
    }

    /// Regression: re-remembering the exact same fact must not duplicate it.
    #[tokio::test]
    async fn execute_does_not_duplicate_an_identical_fact() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool::file_only();
        let ctx = context(&temp_dir);

        tool.execute(serde_json::json!({ "fact": "same fact" }), &ctx)
            .await
            .unwrap();
        let second = tool
            .execute(serde_json::json!({ "fact": "same fact" }), &ctx)
            .await
            .unwrap();

        assert!(second.success);
        assert!(second.output.contains("Already remembered"));
        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert_eq!(contents, "## Memories\n- same fact\n");
    }

    /// Regression: this is the whole point of the tool over session_context's
    /// add_fact - the memory file is keyed by working directory, not session
    /// id, so a second "session" (a fresh ToolExecutionContext, same working
    /// directory) sees facts saved by an earlier one.
    #[tokio::test]
    async fn memory_persists_across_different_sessions_in_the_same_directory() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool::file_only();

        let session_one = context(&temp_dir);
        tool.execute(
            serde_json::json!({ "fact": "remembered in session one" }),
            &session_one,
        )
        .await
        .unwrap();

        // A different session_id (a fresh conversation), same directory.
        let session_two = context(&temp_dir);
        assert_ne!(session_one.session_id, session_two.session_id);

        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert!(contents.contains("remembered in session one"));

        tool.execute(
            serde_json::json!({ "fact": "remembered in session two" }),
            &session_two,
        )
        .await
        .unwrap();

        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert!(contents.contains("remembered in session one"));
        assert!(contents.contains("remembered in session two"));
    }

    #[tokio::test]
    async fn execute_blocked_in_read_only_mode() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool::file_only();
        let mut ctx = context(&temp_dir);
        ctx.read_only_mode = true;

        let result = tool.execute(serde_json::json!({ "fact": "x" }), &ctx).await;
        assert!(matches!(result, Err(ToolError::PermissionDenied(_))));
    }

    #[test]
    fn validate_input_rejects_empty_fact() {
        let tool = SaveMemoryTool::file_only();
        assert!(tool
            .validate_input(&serde_json::json!({ "fact": "   " }))
            .is_err());
    }

    #[test]
    fn append_fact_adds_header_to_a_file_that_lacks_one() {
        let (content, added) = append_fact("- pre-existing line without a header\n", "new fact");
        assert!(added);
        assert_eq!(
            content,
            "## Memories\n- pre-existing line without a header\n- new fact\n"
        );
    }

    /// A context whose session exists, as it does in real use: memories
    /// record their source session, which references the sessions table.
    async fn db_context(db: &crate::db::Database, dir: &TempDir) -> ToolExecutionContext {
        let session = Uuid::new_v4();
        sqlx::query("INSERT INTO sessions (id, created_at, updated_at) VALUES (?, 0, 0)")
            .bind(session.to_string())
            .execute(db.pool())
            .await
            .unwrap();
        ToolExecutionContext::new(session).with_working_directory(dir.path().to_path_buf())
    }

    async fn store_with_db() -> (crate::db::Database, Arc<MemoryStore>) {
        let db = crate::db::Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let store = Arc::new(MemoryStore::new(
            db.pool().clone(),
            crate::config::MemoryConfig::default(),
        ));
        (db, store)
    }

    #[tokio::test]
    async fn database_save_is_stored_announced_and_not_written_to_the_file() {
        let dir = TempDir::new().unwrap();
        let (_db, store) = store_with_db().await;
        let tool = SaveMemoryTool::new(Some(store.clone()));

        let result = tool
            .execute(
                serde_json::json!({"fact": "Uses tokio."}),
                &db_context(&_db, &dir).await,
            )
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(!dir.path().join(".crustly").join("memory.md").exists());
        assert_eq!(store.take_notices().len(), 1);
    }

    #[tokio::test]
    async fn database_save_can_be_global_with_a_category() {
        let dir = TempDir::new().unwrap();
        let (_db, store) = store_with_db().await;
        let tool = SaveMemoryTool::new(Some(store));
        let result = tool
            .execute(
                serde_json::json!({"fact": "Prefers short answers", "scope": "global", "category": "preference"}),
                &db_context(&_db, &dir).await,
            )
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(result.output.contains("global"), "{}", result.output);
    }

    #[tokio::test]
    async fn database_save_refuses_episodes_bad_expiry_and_secrets() {
        let dir = TempDir::new().unwrap();
        let (_db, store) = store_with_db().await;
        let tool = SaveMemoryTool::new(Some(store));
        for input in [
            serde_json::json!({"fact": "Did a thing", "category": "episode"}),
            serde_json::json!({"fact": "Until Friday", "expires_in_days": 0}),
            serde_json::json!({"fact": format!("key {}", ["AKIA", "ABCDEFGHIJKLMNOP"].concat())}),
        ] {
            let result = tool
                .execute(input.clone(), &db_context(&_db, &dir).await)
                .await
                .unwrap();
            assert!(!result.success, "must be refused: {input}");
        }
    }

    #[tokio::test]
    async fn database_replace_with_an_unknown_id_is_an_error() {
        let dir = TempDir::new().unwrap();
        let (_db, store) = store_with_db().await;
        let tool = SaveMemoryTool::new(Some(store));
        let result = tool
            .execute(
                serde_json::json!({"fact": "Uses PostgreSQL", "replaces": "00000000"}),
                &db_context(&_db, &dir).await,
            )
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result
            .error
            .unwrap_or_default()
            .contains("No memory with id"));
    }

    #[tokio::test]
    async fn only_global_saves_with_a_store_need_approval() {
        let (_db, store) = store_with_db().await;
        let with_store = SaveMemoryTool::new(Some(store));
        assert!(
            with_store.requires_approval_for(&serde_json::json!({"fact": "x", "scope": "global"}))
        );
        assert!(!with_store.requires_approval_for(&serde_json::json!({"fact": "x"})));
        assert!(!with_store
            .requires_approval_for(&serde_json::json!({"fact": "x", "scope": "project"})));
        let file_only = SaveMemoryTool::file_only();
        assert!(
            !file_only.requires_approval_for(&serde_json::json!({"fact": "x", "scope": "global"}))
        );
    }
}
