//! Plan Management Tool
//!
//! Allows the LLM to create, update, and manage structured plans for complex tasks.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use crate::tui::plan::{PlanDocument, PlanStatus, PlanTask, TaskType};
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

/// Plan management tool
pub struct PlanTool;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
enum PlanOperation {
    /// Create a new plan
    Create {
        title: String,
        description: String,
        #[serde(default)]
        context: String,
        #[serde(default)]
        risks: Vec<String>,
    },
    /// Add a task to the current plan
    AddTask {
        title: String,
        description: String,
        task_type: String,
        #[serde(default)]
        dependencies: Vec<usize>, // Task order numbers
        #[serde(default = "default_complexity")]
        complexity: u8,
    },
    /// Update plan metadata
    UpdatePlan {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        risks: Option<Vec<String>>,
    },
    /// Mark plan as ready for review
    Finalize,
    /// Get current plan status
    Status,
    /// Export plan to markdown file
    ExportMarkdown {
        #[serde(default)]
        filename: Option<String>,
    },
}

fn default_complexity() -> u8 {
    3
}

/// Validate plan file path for security
/// Prevents symlink attacks and path traversal
fn validate_plan_file_path(path: &Path, working_dir: &Path) -> Result<()> {
    // Check if path is absolute and within working directory
    if !path.starts_with(working_dir) {
        return Err(ToolError::InvalidInput(
            "Plan file must be within working directory".to_string(),
        ));
    }

    // Check for symlinks (security risk)
    if path.exists() {
        let metadata = std::fs::symlink_metadata(path).map_err(ToolError::Io)?;
        if metadata.is_symlink() {
            return Err(ToolError::InvalidInput(
                "Plan file cannot be a symlink (security restriction)".to_string(),
            ));
        }
    }

    // Verify filename matches pattern .crustly_plan_{uuid}.json (no traversal)
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| ToolError::InvalidInput("Invalid plan filename".to_string()))?;

    if !file_name.starts_with(".crustly_plan_") || !file_name.ends_with(".json") {
        return Err(ToolError::InvalidInput(
            "Plan filename must match pattern .crustly_plan_{session_id}.json".to_string(),
        ));
    }

    // Extract and validate UUID portion
    let uuid_part = &file_name[14..file_name.len() - 5]; // Remove ".crustly_plan_" and ".json"
    uuid::Uuid::parse_str(uuid_part).map_err(|_| {
        ToolError::InvalidInput("Plan filename must contain a valid UUID".to_string())
    })?;

    Ok(())
}

/// Maximum plan file size (10MB)
const MAX_PLAN_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Input validation limits
const MAX_TITLE_LENGTH: usize = 200;
const MAX_DESCRIPTION_LENGTH: usize = 5000;
const MAX_CONTEXT_LENGTH: usize = 5000;

/// Validate string input
fn validate_string(s: &str, max_len: usize, field_name: &str) -> Result<()> {
    if s.is_empty() || s.trim().is_empty() {
        return Err(ToolError::InvalidInput(format!(
            "{} cannot be empty",
            field_name
        )));
    }

    if s.len() > max_len {
        return Err(ToolError::InvalidInput(format!(
            "{} exceeds maximum length of {} characters (got {})",
            field_name,
            max_len,
            s.len()
        )));
    }

    Ok(())
}

#[async_trait]
impl Tool for PlanTool {
    fn name(&self) -> &str {
        "plan"
    }

    fn description(&self) -> &str {
        "Manage structured task plans. Use this to break down complex requests into organized, \
         trackable tasks. Create plans, add tasks with dependencies, and finalize for user approval."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["create", "add_task", "update_plan", "finalize", "status", "export_markdown"],
                    "description": "Operation to perform"
                },
                "title": {
                    "type": "string",
                    "description": "Plan or task title (for create/add_task)"
                },
                "description": {
                    "type": "string",
                    "description": "Plan or task description (for create/add_task/update_plan)"
                },
                "context": {
                    "type": "string",
                    "description": "Context and assumptions (for create/update_plan)"
                },
                "risks": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Identified risks and unknowns (for create/update_plan)"
                },
                "task_type": {
                    "type": "string",
                    "enum": ["research", "edit", "create", "delete", "test", "refactor", "documentation", "configuration", "build"],
                    "description": "Type of task (for add_task)"
                },
                "dependencies": {
                    "type": "array",
                    "items": { "type": "integer" },
                    "description": "Task order numbers that must complete first (for add_task)"
                },
                "complexity": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 5,
                    "default": 3,
                    "description": "Task complexity from 1 (simple) to 5 (very complex)"
                },
                "filename": {
                    "type": "string",
                    "description": "Output filename for export_markdown (optional, defaults to PLAN.md)"
                }
            },
            "required": ["operation"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::PlanManagement]
    }

    fn requires_approval(&self) -> bool {
        false // Plan management doesn't require approval, only viewing/creation
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let _: PlanOperation = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let operation: PlanOperation = serde_json::from_value(input)?;

        // Load or create plan state from context (session-scoped)
        let plan_filename = format!(".crustly_plan_{}.json", context.session_id);
        let plan_file = context.working_directory.join(&plan_filename);

        // Security: Validate plan file path
        validate_plan_file_path(&plan_file, &context.working_directory)?;

        // Load existing plan with security checks
        let mut plan: Option<PlanDocument> = if plan_file.exists() {
            // Security: Check file size before reading
            let metadata = tokio::fs::metadata(&plan_file)
                .await
                .map_err(ToolError::Io)?;

            if metadata.len() > MAX_PLAN_FILE_SIZE {
                return Err(ToolError::InvalidInput(format!(
                    "Plan file too large: {} bytes (max: {} bytes)",
                    metadata.len(),
                    MAX_PLAN_FILE_SIZE
                )));
            }

            let content = tokio::fs::read_to_string(&plan_file)
                .await
                .map_err(ToolError::Io)?;

            Some(serde_json::from_str(&content).map_err(|e| {
                ToolError::InvalidInput(format!("Failed to parse plan file: {}", e))
            })?)
        } else {
            None
        };

        let result = match operation {
            PlanOperation::Create {
                title,
                description,
                context: ctx,
                risks,
            } => {
                // Validate inputs
                validate_string(&title, MAX_TITLE_LENGTH, "Plan title")?;
                validate_string(&description, MAX_DESCRIPTION_LENGTH, "Plan description")?;
                if !ctx.is_empty() {
                    validate_string(&ctx, MAX_CONTEXT_LENGTH, "Plan context")?;
                }

                if plan.is_some() {
                    return Ok(ToolResult::error(
                        "A plan already exists. Use 'update_plan' to modify it or 'finalize' to complete it."
                            .to_string(),
                    ));
                }

                let mut new_plan =
                    PlanDocument::new(context.session_id, title.clone(), description);
                new_plan.context = ctx;
                new_plan.risks = risks;
                new_plan.status = PlanStatus::Draft;

                plan = Some(new_plan.clone());

                format!(
                    "✓ Created new plan: '{}'\n\nNext steps:\n\
                     1. Use 'add_task' to add tasks to the plan\n\
                     2. Use 'finalize' when ready for user review",
                    title
                )
            }

            PlanOperation::AddTask {
                title,
                description,
                task_type,
                dependencies,
                complexity,
            } => {
                // Validate inputs
                validate_string(&title, MAX_TITLE_LENGTH, "Task title")?;
                validate_string(&description, MAX_DESCRIPTION_LENGTH, "Task description")?;

                let current_plan = plan.as_mut().ok_or_else(|| {
                    ToolError::InvalidInput(
                        "No active plan. Create a plan first with 'create' operation.".to_string(),
                    )
                })?;

                // Parse task type
                let parsed_type = match task_type.to_lowercase().as_str() {
                    "research" => TaskType::Research,
                    "edit" => TaskType::Edit,
                    "create" => TaskType::Create,
                    "delete" => TaskType::Delete,
                    "test" => TaskType::Test,
                    "refactor" => TaskType::Refactor,
                    "documentation" => TaskType::Documentation,
                    "configuration" => TaskType::Configuration,
                    "build" => TaskType::Build,
                    other => TaskType::Other(other.to_string()),
                };

                let task_order = current_plan.tasks.len() + 1;
                let mut task =
                    PlanTask::new(task_order, title.clone(), description, parsed_type.clone());
                task.complexity = complexity.clamp(1, 5);

                // Validate and convert dependency order numbers to task IDs
                for dep_order in dependencies {
                    if dep_order == 0 {
                        return Err(ToolError::InvalidInput(
                            "Task numbers start at 1, not 0".to_string(),
                        ));
                    }
                    if dep_order >= task_order {
                        return Err(ToolError::InvalidInput(format!(
                            "Task {} cannot depend on task {} (not yet created or would create a cycle)",
                            task_order, dep_order
                        )));
                    }

                    let dep_task = current_plan.tasks.get(dep_order - 1).ok_or_else(|| {
                        ToolError::InvalidInput(format!(
                            "Invalid dependency: task {} does not exist",
                            dep_order
                        ))
                    })?;

                    task.dependencies.push(dep_task.id);
                }

                current_plan.add_task(task);

                format!(
                    "✓ Added task #{}: '{}'\n  Type: {:?} | Complexity: {}★\n  Total tasks: {}",
                    task_order,
                    title,
                    parsed_type,
                    complexity,
                    current_plan.tasks.len()
                )
            }

            PlanOperation::UpdatePlan {
                title,
                description,
                context: ctx,
                risks,
            } => {
                let current_plan = plan.as_mut().ok_or_else(|| {
                    ToolError::InvalidInput("No active plan to update.".to_string())
                })?;

                if let Some(t) = title {
                    current_plan.title = t;
                }
                if let Some(d) = description {
                    current_plan.description = d;
                }
                if let Some(c) = ctx {
                    current_plan.context = c;
                }
                if let Some(r) = risks {
                    current_plan.risks = r;
                }
                current_plan.updated_at = Utc::now();

                "✓ Plan updated successfully".to_string()
            }

            PlanOperation::Finalize => {
                let current_plan = plan.as_mut().ok_or_else(|| {
                    ToolError::InvalidInput("No active plan to finalize.".to_string())
                })?;

                if current_plan.tasks.is_empty() {
                    return Ok(ToolResult::error(
                        "Cannot finalize plan with no tasks. Add tasks first.".to_string(),
                    ));
                }

                // Validate dependencies before finalizing
                if let Err(e) = current_plan.validate_dependencies() {
                    return Ok(ToolResult::error(format!(
                        "Cannot finalize plan: {}\n\n\
                         Please fix the dependency issues before finalizing.",
                        e
                    )));
                }

                current_plan.status = PlanStatus::PendingApproval;
                current_plan.updated_at = Utc::now();

                format!(
                    "✓ Plan finalized and ready for review!\n\n\
                     📋 Plan: {}\n\
                     📝 {} tasks ready for execution\n\n\
                     The plan will now be presented to the user for approval.\n\
                     User can approve with Ctrl+A or reject with Ctrl+R.",
                    current_plan.title,
                    current_plan.tasks.len()
                )
            }

            PlanOperation::Status => {
                if let Some(current_plan) = &plan {
                    format!(
                        "📋 Current Plan Status\n\n\
                         Title: {}\n\
                         Status: {:?}\n\
                         Tasks: {}\n\
                         Created: {}\n\
                         Updated: {}",
                        current_plan.title,
                        current_plan.status,
                        current_plan.tasks.len(),
                        current_plan.created_at.format("%Y-%m-%d %H:%M:%S"),
                        current_plan.updated_at.format("%Y-%m-%d %H:%M:%S")
                    )
                } else {
                    "No active plan. Create one with 'create' operation.".to_string()
                }
            }

            PlanOperation::ExportMarkdown { filename } => {
                let current_plan = plan.as_ref().ok_or_else(|| {
                    ToolError::InvalidInput("No active plan to export.".to_string())
                })?;

                // Generate markdown content
                let mut markdown = String::new();
                markdown.push_str(&format!("# {}\n\n", current_plan.title));
                markdown.push_str(&format!("{}\n\n", current_plan.description));

                if !current_plan.context.is_empty() {
                    markdown.push_str("## Context\n\n");
                    markdown.push_str(&format!("{}\n\n", current_plan.context));
                }

                if !current_plan.risks.is_empty() {
                    markdown.push_str("## Risks & Considerations\n\n");
                    for risk in &current_plan.risks {
                        markdown.push_str(&format!("- {}\n", risk));
                    }
                    markdown.push_str("\n");
                }

                markdown.push_str("## Tasks\n\n");

                for task in &current_plan.tasks {
                    markdown.push_str(&format!("### Task {}: {}\n\n", task.order, task.title));
                    markdown.push_str(&format!("**Type:** {:?} | **Complexity:** {}★\n\n", task.task_type, task.complexity));

                    if !task.dependencies.is_empty() {
                        let dep_orders: Vec<String> = task.dependencies
                            .iter()
                            .filter_map(|dep_id| {
                                current_plan.tasks.iter()
                                    .find(|t| &t.id == dep_id)
                                    .map(|t| t.order.to_string())
                            })
                            .collect();
                        markdown.push_str(&format!("**Dependencies:** Task(s) {}\n\n", dep_orders.join(", ")));
                    }

                    markdown.push_str("**Implementation Steps:**\n\n");
                    markdown.push_str(&format!("{}\n\n", task.description));
                    markdown.push_str("---\n\n");
                }

                markdown.push_str(&format!("\n*Plan created: {}*\n", current_plan.created_at.format("%Y-%m-%d %H:%M:%S")));
                markdown.push_str(&format!("*Last updated: {}*\n", current_plan.updated_at.format("%Y-%m-%d %H:%M:%S")));

                // Determine output filename
                let output_filename = filename.as_deref().unwrap_or("PLAN.md");
                let output_path = context.working_directory.join(output_filename);

                // Validate output path (security check)
                if !output_path.starts_with(&context.working_directory) {
                    return Err(ToolError::InvalidInput(
                        "Output file must be within working directory".to_string(),
                    ));
                }

                // Check if file already exists
                if output_path.exists() {
                    return Err(ToolError::InvalidInput(
                        format!("File '{}' already exists. Please choose a different filename or delete the existing file first.", output_filename)
                    ));
                }

                // Write markdown file
                tokio::fs::write(&output_path, markdown)
                    .await
                    .map_err(ToolError::Io)?;

                format!(
                    "✓ Plan exported to '{}'!\n\n\
                     📄 {} tasks documented in detail\n\n\
                     Review the file to see all implementation steps.\n\
                     Use this as a reference when executing the plan.",
                    output_filename,
                    current_plan.tasks.len()
                )
            }
        };

        // Save plan to file with atomic write
        if let Some(ref current_plan) = plan {
            let json = serde_json::to_string_pretty(current_plan)
                .map_err(|e| ToolError::InvalidInput(format!("Failed to serialize plan: {}", e)))?;

            // Atomic write: write to temp file, then rename
            let temp_file = plan_file.with_extension("tmp");

            // Write to temp file
            tokio::fs::write(&temp_file, &json)
                .await
                .map_err(ToolError::Io)?;

            // Atomic rename (ensures consistency even if interrupted)
            tokio::fs::rename(&temp_file, &plan_file)
                .await
                .map_err(ToolError::Io)?;
        }

        Ok(ToolResult::success(result))
    }
}

#[cfg(test)]
#[path = "plan_tool_security_tests.rs"]
mod plan_tool_security_tests;
