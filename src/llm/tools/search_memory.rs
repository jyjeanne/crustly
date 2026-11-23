//! Search tool for long-term memory (docs/improve-memory.md §10.2).
//!
//! Read-only, so it needs no approval. It returns memories in the same line
//! format the recall block uses, for when the block did not include what the
//! model needs. Only the current project's and the global memories are ever
//! searched.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use crate::memory::model::{Memory, MemoryCategory, MemoryScope};
use crate::memory::recall::short_id;
use crate::memory::{MemoryStore, SearchScope};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;

const DEFAULT_LIMIT: usize = 8;
const MAX_LIMIT: usize = 20;

pub struct SearchMemoryTool {
    store: Arc<MemoryStore>,
}

impl SearchMemoryTool {
    pub fn new(store: Arc<MemoryStore>) -> Self {
        Self { store }
    }
}

#[derive(Debug, Deserialize)]
struct SearchMemoryInput {
    query: String,
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    limit: Option<usize>,
}

/// One result line, in the recall block's format.
fn result_line(memory: &Memory) -> String {
    let scope = match memory.scope {
        MemoryScope::Global => "global",
        MemoryScope::Project(_) => "project",
    };
    format!(
        "- [{} · {scope} · m:{}] {}",
        memory.category.as_str(),
        short_id(memory),
        memory.content
    )
}

#[async_trait]
impl Tool for SearchMemoryTool {
    fn name(&self) -> &str {
        "search_memory"
    }

    fn description(&self) -> &str {
        "Search long-term memory: facts and preferences saved in earlier sessions, for this \
         project and for the user globally. Use it when the memories already recalled with the \
         request do not cover what you need. Recalled memories may be out of date; check \
         anything that matters against the repository."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Words to look for, e.g. 'database storage' or 'async runtime'."
                },
                "scope": {
                    "type": "string",
                    "enum": ["project", "global", "all"],
                    "description": "Which memories to search. Default: all."
                },
                "category": {
                    "type": "string",
                    "enum": ["preference", "convention", "decision", "fact", "lesson", "episode"]
                },
                "limit": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": MAX_LIMIT
                }
            },
            "required": ["query"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        Vec::new()
    }

    fn requires_approval(&self) -> bool {
        false
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: SearchMemoryInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {e}")))?;
        if input.query.trim().is_empty() {
            return Err(ToolError::InvalidInput("query cannot be empty".to_string()));
        }
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: SearchMemoryInput = serde_json::from_value(input)?;
        let scope = match input.scope.as_deref().unwrap_or("all") {
            "all" => SearchScope::All,
            "global" => SearchScope::Global,
            "project" => SearchScope::Project,
            other => {
                return Ok(ToolResult::error(format!(
                    "scope must be project, global or all, not {other}"
                )))
            }
        };
        let category = match input.category.as_deref() {
            None => None,
            Some(text) => match MemoryCategory::from_str(text) {
                Ok(category) => Some(category),
                Err(_) => return Ok(ToolResult::error(format!("unknown category {text}"))),
            },
        };
        let limit = input.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

        let project = crate::memory::project::identify(&context.working_directory);
        let found = match self
            .store
            .search(&project, &input.query, scope, category, limit)
            .await
        {
            Ok(found) => found,
            Err(err) => return Ok(ToolResult::error(format!("Search failed: {err}"))),
        };
        if found.is_empty() {
            return Ok(ToolResult::success(format!(
                "No saved memories match '{}'.",
                input.query
            )));
        }
        let lines: Vec<String> = found.iter().map(result_line).collect();
        Ok(ToolResult::success(lines.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MemoryConfig;
    use crate::db::Database;
    use crate::memory::model::{MemorySource, NewMemory, SaveOutcome};
    use tempfile::TempDir;
    use uuid::Uuid;

    async fn setup(dir: &TempDir) -> (Database, SearchMemoryTool, ToolExecutionContext) {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let store = Arc::new(MemoryStore::new(db.pool().clone(), MemoryConfig::default()));
        let session = Uuid::new_v4();
        sqlx::query("INSERT INTO sessions (id, created_at, updated_at) VALUES (?, 0, 0)")
            .bind(session.to_string())
            .execute(db.pool())
            .await
            .unwrap();
        let context =
            ToolExecutionContext::new(session).with_working_directory(dir.path().to_path_buf());
        let project = crate::memory::project::identify(dir.path());
        store.register_project(&project).await.unwrap();
        (db, SearchMemoryTool::new(store), context)
    }

    async fn save(
        tool: &SearchMemoryTool,
        scope: MemoryScope,
        content: &str,
        category: MemoryCategory,
    ) {
        let outcome = tool
            .store
            .save(NewMemory {
                content: content.to_string(),
                scope,
                category,
                source: MemorySource::User,
                importance: 3,
                replaces: None,
                session_id: None,
                expires_at: None,
            })
            .await
            .unwrap();
        assert!(matches!(outcome, SaveOutcome::Created(_)), "{outcome:?}");
    }

    #[tokio::test]
    async fn finds_project_and_global_memories_by_words() {
        let dir = TempDir::new().unwrap();
        let (_db, tool, context) = setup(&dir).await;
        let project = crate::memory::project::identify(dir.path());
        save(
            &tool,
            MemoryScope::Project(project.id.clone()),
            "Uses PostgreSQL for storage",
            MemoryCategory::Fact,
        )
        .await;
        save(
            &tool,
            MemoryScope::Global,
            "Prefers Tokio for storage code",
            MemoryCategory::Preference,
        )
        .await;

        let result = tool
            .execute(serde_json::json!({"query": "storage"}), &context)
            .await
            .unwrap();
        assert!(result.success);
        assert!(
            result.output.contains("PostgreSQL") && result.output.contains("Tokio"),
            "{}",
            result.output
        );
        assert!(
            result.output.contains("- [fact · project · m:"),
            "{}",
            result.output
        );
    }

    #[tokio::test]
    async fn scope_and_category_narrow_the_results() {
        let dir = TempDir::new().unwrap();
        let (_db, tool, context) = setup(&dir).await;
        let project = crate::memory::project::identify(dir.path());
        save(
            &tool,
            MemoryScope::Project(project.id.clone()),
            "Uses PostgreSQL for storage",
            MemoryCategory::Fact,
        )
        .await;
        save(
            &tool,
            MemoryScope::Global,
            "Prefers Tokio for storage code",
            MemoryCategory::Preference,
        )
        .await;

        let global_only = tool
            .execute(
                serde_json::json!({"query": "storage", "scope": "global"}),
                &context,
            )
            .await
            .unwrap();
        assert!(global_only.output.contains("Tokio") && !global_only.output.contains("PostgreSQL"));

        let decisions = tool
            .execute(
                serde_json::json!({"query": "storage", "category": "decision"}),
                &context,
            )
            .await
            .unwrap();
        assert!(
            decisions.output.starts_with("No saved memories"),
            "{}",
            decisions.output
        );
    }

    #[tokio::test]
    async fn never_returns_another_projects_memories() {
        let dir = TempDir::new().unwrap();
        let (_db, tool, context) = setup(&dir).await;
        let other = crate::memory::project::ProjectIdentity {
            id: "some-other-project".to_string(),
            root: std::path::PathBuf::from("/elsewhere"),
            name: "elsewhere".to_string(),
            remote_url: None,
        };
        tool.store.register_project(&other).await.unwrap();
        save(
            &tool,
            MemoryScope::Project("some-other-project".into()),
            "Uses Redis caching",
            MemoryCategory::Fact,
        )
        .await;

        let result = tool
            .execute(serde_json::json!({"query": "redis"}), &context)
            .await
            .unwrap();
        assert!(
            result.output.starts_with("No saved memories"),
            "{}",
            result.output
        );
    }

    #[tokio::test]
    async fn bad_arguments_are_errors_and_search_needs_no_approval() {
        let dir = TempDir::new().unwrap();
        let (_db, tool, context) = setup(&dir).await;
        assert!(!tool.requires_approval());
        assert!(tool.capabilities().is_empty());
        let bad = tool
            .execute(
                serde_json::json!({"query": "x", "scope": "everywhere"}),
                &context,
            )
            .await
            .unwrap();
        assert!(!bad.success);
        assert!(tool
            .validate_input(&serde_json::json!({"query": "  "}))
            .is_err());
    }
}
