//! Todo List Tool
//!
//! Lightweight session-scoped todo list for tracking agent work items.
//! Backed by a JSON file in the session's working directory.
//! Complements the heavier `task` tool with a simpler, read-write-at-once interface.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use tokio::fs;

const TODO_FILE: &str = ".crustly/todos.json";

/// Todo list management tool (write and read in a single atomic operation)
pub struct TodoWriteTool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "pending"),
            TodoStatus::InProgress => write!(f, "in_progress"),
            TodoStatus::Completed => write!(f, "completed"),
            TodoStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TodoPriority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for TodoPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoPriority::Low => write!(f, "low"),
            TodoPriority::Medium => write!(f, "medium"),
            TodoPriority::High => write!(f, "high"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: TodoStatus,
    pub priority: TodoPriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TodoStore {
    todos: Vec<TodoItem>,
}

impl TodoStore {
    async fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path).await.map_err(ToolError::Io)?;
        serde_json::from_str(&content)
            .map_err(|e| ToolError::Execution(format!("Failed to parse todo store: {}", e)))
    }

    async fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(ToolError::Io)?;
        }
        let content = serde_json::to_string_pretty(self).map_err(ToolError::Json)?;
        fs::write(path, content).await.map_err(ToolError::Io)
    }
}

/// Input for reading todos
#[derive(Debug, Deserialize)]
struct ReadInput {
    // no fields required
}

/// Input for writing / overwriting the full todo list
#[derive(Debug, Deserialize)]
struct WriteInput {
    todos: Vec<TodoItemInput>,
}

#[derive(Debug, Deserialize)]
struct TodoItemInput {
    id: String,
    content: String,
    status: TodoStatus,
    #[serde(default = "default_priority")]
    priority: TodoPriority,
}

fn default_priority() -> TodoPriority {
    TodoPriority::Medium
}

/// Unified input: either a `read` or `write` operation
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
enum TodoInput {
    Read(ReadInput),
    Write(WriteInput),
}

fn render_todos(todos: &[TodoItem]) -> String {
    if todos.is_empty() {
        return "No todos found.".to_string();
    }

    let mut out = String::new();
    for todo in todos {
        let marker = match todo.status {
            TodoStatus::Completed => "[x]",
            TodoStatus::InProgress => "[~]",
            TodoStatus::Cancelled => "[-]",
            TodoStatus::Pending => "[ ]",
        };
        let prio = match todo.priority {
            TodoPriority::High => "!",
            TodoPriority::Medium => " ",
            TodoPriority::Low => ".",
        };
        out.push_str(&format!(
            "{} {} [{}] {}\n",
            marker, prio, todo.id, todo.content
        ));
    }
    out
}

#[async_trait]
impl Tool for TodoWriteTool {
    fn name(&self) -> &str {
        "todo_write"
    }

    fn description(&self) -> &str {
        "Manage a persistent todo list for the current session. Use action=\"read\" to list all \
         todos, or action=\"write\" with a full list of todos to overwrite the store. \
         Each todo has an id, content, status (pending/in_progress/completed/cancelled), \
         and priority (low/medium/high)."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Operation to perform",
                    "enum": ["read", "write"]
                },
                "todos": {
                    "type": "array",
                    "description": "Full list of todos to write (required for action=write)",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string",
                                "description": "Unique identifier for the todo item"
                            },
                            "content": {
                                "type": "string",
                                "description": "Todo item description"
                            },
                            "status": {
                                "type": "string",
                                "enum": ["pending", "in_progress", "completed", "cancelled"]
                            },
                            "priority": {
                                "type": "string",
                                "enum": ["low", "medium", "high"],
                                "default": "medium"
                            }
                        },
                        "required": ["id", "content", "status"]
                    }
                }
            },
            "required": ["action"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles]
    }

    fn requires_approval(&self) -> bool {
        false // Writing todos is low-risk
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        serde_json::from_value::<TodoInput>(input.clone())
            .map(|_| ())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let todo_input: TodoInput = serde_json::from_value(input)?;
        let todo_path = context.working_directory.join(TODO_FILE);

        match todo_input {
            TodoInput::Read(_) => {
                let store = TodoStore::load(&todo_path).await?;
                Ok(ToolResult::success(render_todos(&store.todos)).with_metadata(
                    "count".to_string(),
                    store.todos.len().to_string(),
                ))
            }
            TodoInput::Write(write_input) => {
                if context.read_only_mode {
                    return Err(ToolError::PermissionDenied(
                        "Cannot write todos in read-only (plan) mode".to_string(),
                    ));
                }

                let now = Utc::now();

                // Load existing to preserve created_at timestamps
                let existing = TodoStore::load(&todo_path).await.unwrap_or_default();
                let existing_map: std::collections::HashMap<_, _> =
                    existing.todos.into_iter().map(|t| (t.id.clone(), t)).collect();

                let todos: Vec<TodoItem> = write_input
                    .todos
                    .into_iter()
                    .map(|item| {
                        let created_at = existing_map
                            .get(&item.id)
                            .map(|e| e.created_at)
                            .unwrap_or(now);
                        TodoItem {
                            id: item.id,
                            content: item.content,
                            status: item.status,
                            priority: item.priority,
                            created_at,
                            updated_at: now,
                        }
                    })
                    .collect();

                let count = todos.len();
                let store = TodoStore { todos };
                store.save(&todo_path).await?;

                Ok(ToolResult::success(format!("Todo list updated ({} items).", count))
                    .with_metadata("count".to_string(), count.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_todos_empty() {
        assert_eq!(render_todos(&[]), "No todos found.");
    }

    #[test]
    fn test_render_todos_completed() {
        let item = TodoItem {
            id: "1".to_string(),
            content: "Fix the bug".to_string(),
            status: TodoStatus::Completed,
            priority: TodoPriority::High,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let rendered = render_todos(&[item]);
        assert!(rendered.starts_with("[x]"));
        assert!(rendered.contains("Fix the bug"));
    }

    #[test]
    fn test_validate_read_action() {
        let tool = TodoWriteTool;
        let result = tool.validate_input(&serde_json::json!({ "action": "read" }));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_write_requires_todos() {
        let tool = TodoWriteTool;
        let result = tool.validate_input(&serde_json::json!({ "action": "write" }));
        // Missing "todos" field — should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_write_with_todos() {
        let tool = TodoWriteTool;
        let result = tool.validate_input(&serde_json::json!({
            "action": "write",
            "todos": [
                { "id": "1", "content": "Do something", "status": "pending" }
            ]
        }));
        assert!(result.is_ok());
    }
}
