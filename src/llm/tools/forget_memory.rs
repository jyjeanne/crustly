//! Forget tool for long-term memory (docs/improve-memory.md §10.3).
//!
//! Always asks for approval, and the prompt shows the content of each memory
//! that would be removed. The model names memories by id (or a unique id
//! prefix) that it found with `search_memory`; there is no deletion by
//! query, so a vague request cannot delete in bulk. Every id is resolved
//! before anything is deleted: one unknown or ambiguous id removes nothing.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use crate::memory::model::Memory;
use crate::memory::recall::short_id;
use crate::memory::MemoryStore;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

/// Most memories one call may forget.
const MAX_IDS: usize = 10;

pub struct ForgetMemoryTool {
    store: Arc<MemoryStore>,
}

impl ForgetMemoryTool {
    pub fn new(store: Arc<MemoryStore>) -> Self {
        Self { store }
    }
}

#[derive(Debug, Deserialize)]
struct ForgetMemoryInput {
    ids: Vec<String>,
}

/// Resolve every id to exactly one memory, or say which one could not be.
async fn resolve(store: &MemoryStore, ids: &[String]) -> std::result::Result<Vec<Memory>, String> {
    if ids.is_empty() || ids.len() > MAX_IDS {
        return Err(format!("give between 1 and {MAX_IDS} memory ids"));
    }
    let mut found: Vec<Memory> = Vec::new();
    for prefix in ids {
        let matches = store
            .find(prefix)
            .await
            .map_err(|err| format!("lookup failed: {err}"))?;
        match matches.len() {
            0 => return Err(format!("no memory with id {prefix}")),
            1 => {
                let Some(memory) = matches.into_iter().next() else {
                    continue;
                };
                if !found.iter().any(|m| m.id == memory.id) {
                    found.push(memory);
                }
            }
            _ => {
                return Err(format!(
                    "the id {prefix} matches more than one memory; use a longer prefix"
                ))
            }
        }
    }
    Ok(found)
}

fn describe(memory: &Memory) -> String {
    let scope = match memory.scope {
        crate::memory::model::MemoryScope::Global => "global",
        crate::memory::model::MemoryScope::Project(_) => "project",
    };
    format!(
        "- [{} · {scope} · m:{}] {}",
        memory.category.as_str(),
        short_id(memory),
        memory.content
    )
}

#[async_trait]
impl Tool for ForgetMemoryTool {
    fn name(&self) -> &str {
        "forget_memory"
    }

    fn description(&self) -> &str {
        "Permanently delete saved memories by id, when the user asks to forget them. Find the \
         ids with search_memory first. The user must approve each deletion, and the memories \
         are removed for good."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": { "type": "string" },
                    "minItems": 1,
                    "maxItems": MAX_IDS,
                    "description": "Ids, or unique id prefixes, of the memories to forget."
                }
            },
            "required": ["ids"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles]
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: ForgetMemoryInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {e}")))?;
        if input.ids.is_empty() || input.ids.len() > MAX_IDS {
            return Err(ToolError::InvalidInput(format!(
                "give between 1 and {MAX_IDS} memory ids"
            )));
        }
        Ok(())
    }

    async fn approval_preview(&self, input: &Value) -> Option<String> {
        let input: ForgetMemoryInput = serde_json::from_value(input.clone()).ok()?;
        Some(match resolve(&self.store, &input.ids).await {
            Ok(memories) => {
                let lines: Vec<String> = memories.iter().map(describe).collect();
                format!("Forget these memories for good:\n{}", lines.join("\n"))
            }
            Err(reason) => format!("Cannot forget: {reason}"),
        })
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        // Plan mode changes nothing until the plan is approved (as save_memory).
        if context.read_only_mode {
            return Err(ToolError::PermissionDenied(
                "Cannot forget memories in read-only (plan) mode".to_string(),
            ));
        }
        let input: ForgetMemoryInput = serde_json::from_value(input)?;
        let memories = match resolve(&self.store, &input.ids).await {
            Ok(memories) => memories,
            Err(reason) => return Ok(ToolResult::error(reason)),
        };
        for memory in &memories {
            if let Err(err) = self.store.forget(memory.id, "model").await {
                return Ok(ToolResult::error(format!(
                    "Could not forget a memory: {err}"
                )));
            }
        }
        let lines: Vec<String> = memories.iter().map(describe).collect();
        Ok(ToolResult::success(format!(
            "Forgot {} memor{}:\n{}",
            memories.len(),
            if memories.len() == 1 { "y" } else { "ies" },
            lines.join("\n")
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MemoryConfig;
    use crate::db::Database;
    use crate::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
    use tempfile::TempDir;
    use uuid::Uuid;

    async fn setup() -> (Database, Arc<MemoryStore>) {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let store = Arc::new(MemoryStore::new(db.pool().clone(), MemoryConfig::default()));
        (db, store)
    }

    async fn save(store: &MemoryStore, content: &str) -> Uuid {
        match store
            .save(NewMemory {
                content: content.to_string(),
                scope: MemoryScope::Global,
                category: MemoryCategory::Preference,
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

    fn context() -> ToolExecutionContext {
        ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(TempDir::new().unwrap().path().to_path_buf())
    }

    #[tokio::test]
    async fn the_preview_shows_each_memory_before_anything_is_deleted() {
        let (_db, store) = setup().await;
        let id = save(&store, "Prefers short answers").await;
        let tool = ForgetMemoryTool::new(store.clone());
        let input = serde_json::json!({"ids": [id.simple().to_string()[..8].to_string()]});

        let preview = tool.approval_preview(&input).await.unwrap();
        assert!(preview.contains("Prefers short answers"), "{preview}");
        assert!(
            store.get(id).await.unwrap().is_some(),
            "a preview deletes nothing"
        );
        assert!(tool.requires_approval_for(&input));
    }

    #[tokio::test]
    async fn forgetting_removes_the_memories_and_records_the_actor() {
        let (db, store) = setup().await;
        let id = save(&store, "Prefers short answers").await;
        let tool = ForgetMemoryTool::new(store.clone());
        let result = tool
            .execute(
                serde_json::json!({"ids": [id.simple().to_string()[..8].to_string()]}),
                &context(),
            )
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(store.get(id).await.unwrap().is_none());

        let actor: String = sqlx::query_scalar(
            "SELECT actor FROM memory_events WHERE memory_id = ? AND event = 'forgotten'",
        )
        .bind(id.to_string())
        .fetch_one(db.pool())
        .await
        .unwrap();
        assert_eq!(actor, "model");
    }

    #[tokio::test]
    async fn one_unknown_id_deletes_nothing() {
        let (_db, store) = setup().await;
        let id = save(&store, "Prefers short answers").await;
        let tool = ForgetMemoryTool::new(store.clone());
        let result = tool
            .execute(
                serde_json::json!({"ids": [id.simple().to_string(), "00000000"]}),
                &context(),
            )
            .await
            .unwrap();
        assert!(!result.success);
        assert!(
            store.get(id).await.unwrap().is_some(),
            "the valid id is kept too"
        );
    }

    #[tokio::test]
    async fn too_many_ids_are_refused() {
        let (_db, store) = setup().await;
        let tool = ForgetMemoryTool::new(store);
        let ids: Vec<String> = (0..11).map(|i| format!("{i:08x}")).collect();
        assert!(tool
            .validate_input(&serde_json::json!({"ids": ids}))
            .is_err());
    }
}
