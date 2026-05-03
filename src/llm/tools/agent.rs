//! Agent Tool
//!
//! Spawns a background sub-agent session to handle a focused task asynchronously.
//! The caller gets an immediate response (manifest JSON) and the sub-agent runs
//! independently, writing its output to `.crustly/agents/<id>.md`.
//!
//! The actual launch is delegated to `SubAgentLauncher` stored in the execution
//! context so this tool stays decoupled from the agent service internals.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

const AGENTS_DIR: &str = ".crustly/agents";

pub struct AgentTool;

#[derive(Debug, Deserialize)]
struct AgentInput {
    /// Short description of what the sub-agent will do
    description: String,
    /// Full task prompt for the sub-agent
    prompt: String,
    /// Optional sub-agent type hint (e.g. "general-purpose", "Explore", "Plan")
    subagent_type: Option<String>,
    /// Optional human-readable name for the agent (defaults to slug of description)
    name: Option<String>,
}

#[derive(Debug, Serialize)]
struct AgentManifest {
    #[serde(rename = "agentId")]
    agent_id: String,
    name: String,
    description: String,
    #[serde(rename = "subagentType", skip_serializing_if = "Option::is_none")]
    subagent_type: Option<String>,
    status: String,
    #[serde(rename = "outputFile")]
    output_file: String,
    #[serde(rename = "manifestFile")]
    manifest_file: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[async_trait]
impl Tool for AgentTool {
    fn name(&self) -> &str {
        "agent"
    }

    fn description(&self) -> &str {
        "Spawn a background sub-agent to handle a focused task asynchronously. \
         The sub-agent receives a prompt, runs independently, and writes its output to \
         .crustly/agents/<id>.md. Returns a manifest with the agent ID and output file path \
         immediately, before the sub-agent has finished."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "Short description of the sub-agent task (3-5 words)"
                },
                "prompt": {
                    "type": "string",
                    "description": "Full task prompt to send to the sub-agent"
                },
                "subagent_type": {
                    "type": "string",
                    "description": "Optional sub-agent type hint: 'general-purpose', 'Explore', 'Plan'",
                    "enum": ["general-purpose", "Explore", "Plan", "Verification"]
                },
                "name": {
                    "type": "string",
                    "description": "Optional human-readable name for the agent"
                }
            },
            "required": ["description", "prompt"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles]
    }

    fn requires_approval(&self) -> bool {
        false
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: AgentInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if input.description.trim().is_empty() {
            return Err(ToolError::InvalidInput("description must not be empty".to_string()));
        }
        if input.prompt.trim().is_empty() {
            return Err(ToolError::InvalidInput("prompt must not be empty".to_string()));
        }
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        if context.read_only_mode {
            return Err(ToolError::PermissionDenied(
                "Cannot spawn sub-agents in read-only (plan) mode".to_string(),
            ));
        }

        let input: AgentInput = serde_json::from_value(input)?;

        let agent_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();
        let name = {
            let candidate = input
                .name
                .as_deref()
                .map(slugify)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| slugify(&input.description));
            if candidate.is_empty() {
                agent_id.to_string()
            } else {
                candidate
            }
        };

        let agents_dir = context.working_directory.join(AGENTS_DIR);
        tokio::fs::create_dir_all(&agents_dir)
            .await
            .map_err(ToolError::Io)?;

        let output_file = agents_dir.join(format!("{}.md", agent_id));
        let manifest_file = agents_dir.join(format!("{}.json", agent_id));

        // Write the task description to the output file
        let header = format!(
            "# Agent Task\n\n- id: {agent_id}\n- name: {name}\n- description: {}\n- subagent_type: {}\n- created_at: {created_at}\n\n## Prompt\n\n{}\n",
            input.description,
            input.subagent_type.as_deref().unwrap_or("general-purpose"),
            input.prompt
        );
        tokio::fs::write(&output_file, &header)
            .await
            .map_err(ToolError::Io)?;

        let manifest = AgentManifest {
            agent_id: agent_id.to_string(),
            name,
            description: input.description.clone(),
            subagent_type: input.subagent_type.clone(),
            status: "running".to_string(),
            output_file: output_file.display().to_string(),
            manifest_file: manifest_file.display().to_string(),
            created_at: created_at.clone(),
        };

        let manifest_json = serde_json::to_string_pretty(&manifest).map_err(ToolError::Json)?;
        // If the manifest write fails, clean up the output file so no orphan is left on disk.
        if let Err(e) = tokio::fs::write(&manifest_file, &manifest_json).await {
            let _ = tokio::fs::remove_file(&output_file).await;
            return Err(ToolError::Io(e));
        }

        // Delegate actual sub-agent launch to the injected launcher, if available.
        // Note: AgentServiceLauncher::launch always returns Ok(()) — it fire-and-forgets
        // via tokio::spawn. The error arm below is for future launcher implementations
        // that may perform synchronous pre-flight checks before spawning.
        if let Some(launcher) = &context.sub_agent_launcher {
            if let Err(e) = launcher.launch(agent_id, &input.description, &input.prompt).await {
                let _ = tokio::fs::remove_file(&output_file).await;
                let _ = tokio::fs::remove_file(&manifest_file).await;
                return Err(ToolError::Execution(format!("failed to spawn sub-agent: {e}")));
            }
        } else {
            tracing::debug!(
                agent_id = %agent_id,
                "No SubAgentLauncher wired — agent manifest written but no sub-process started"
            );
        }

        Ok(ToolResult::success(manifest_json)
            .with_metadata("agent_id".to_string(), agent_id.to_string())
            .with_metadata("output_file".to_string(), output_file.display().to_string()))
    }
}

/// Convert a free-form string into a short kebab-case slug.
fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Search codebase"), "search-codebase");
        assert_eq!(slugify("  spaces  "), "spaces");
        assert_eq!(slugify("Multi---hyphens"), "multi-hyphens");
    }

    #[test]
    fn test_validate_empty_description() {
        let tool = AgentTool;
        let result =
            tool.validate_input(&serde_json::json!({ "description": "", "prompt": "do it" }));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_prompt() {
        let tool = AgentTool;
        let result = tool.validate_input(
            &serde_json::json!({ "description": "search code", "prompt": "  " }),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_input() {
        let tool = AgentTool;
        let result = tool.validate_input(&serde_json::json!({
            "description": "search code",
            "prompt": "Find all usages of ToolRegistry"
        }));
        assert!(result.is_ok());
    }
}
