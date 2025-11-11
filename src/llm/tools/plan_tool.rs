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
}

fn default_complexity() -> u8 {
    3
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
                    "enum": ["create", "add_task", "update_plan", "finalize", "status"],
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

        // Load or create plan state from context
        // For now, we'll store plan in a JSON file in the working directory
        let plan_file = context.working_directory.join(".crustly_plan.json");

        let mut plan: Option<PlanDocument> = if plan_file.exists() {
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

                // Convert dependency order numbers to task IDs
                for dep_order in dependencies {
                    if dep_order > 0 && dep_order < task_order {
                        if let Some(dep_task) = current_plan.tasks.get(dep_order - 1) {
                            task.dependencies.push(dep_task.id);
                        }
                    }
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
        };

        // Save plan to file
        if let Some(ref current_plan) = plan {
            let json = serde_json::to_string_pretty(current_plan)
                .map_err(|e| ToolError::InvalidInput(format!("Failed to serialize plan: {}", e)))?;
            tokio::fs::write(&plan_file, json)
                .await
                .map_err(ToolError::Io)?;
        }

        Ok(ToolResult::success(result))
    }
}
