//! Agent Tool
//!
//! Hands a focused task to a sub-agent: a separate agent session with its
//! own context window, its own tool set and, optionally, its own model (see
//! the profiles in `llm::agent::subagents`). The parent sees only the
//! sub-agent's final answer, so a long exploration costs the parent a few
//! hundred tokens instead of every file the sub-agent read.
//!
//! By default the call waits for the answer (`action: "run"`). A sub-agent
//! can also be started in the background and collected later. Running,
//! queueing and cancelling is delegated to the `SubAgentLauncher` in the
//! execution context, so this tool stays decoupled from the agent service.

use super::error::{Result, ToolError};
use super::r#trait::{
    SubAgentReport, SubAgentStatus, SubAgentTask, Tool, ToolCapability, ToolExecutionContext,
    ToolResult,
};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

pub struct AgentTool;

#[derive(Debug, Deserialize, Default)]
struct AgentInput {
    /// run (default) | start | status | result | cancel | list
    #[serde(default)]
    action: Option<String>,
    /// Short description of what the sub-agent will do
    #[serde(default)]
    description: Option<String>,
    /// Full task prompt for the sub-agent
    #[serde(default)]
    prompt: Option<String>,
    /// Profile to run the sub-agent with
    #[serde(default)]
    profile: Option<String>,
    /// Older type hint ("general-purpose", "Explore", "Plan", "Verification"),
    /// mapped to a profile when `profile` is absent
    #[serde(default)]
    subagent_type: Option<String>,
    /// Sub-agent id, for status/result/cancel
    #[serde(default)]
    id: Option<String>,
}

#[async_trait]
impl Tool for AgentTool {
    fn name(&self) -> &str {
        "agent"
    }

    fn description(&self) -> &str {
        "Hand a focused task to a sub-agent with its own context window, and get its answer \
         back. Use it for searches or investigations that would read many files, so only the \
         findings come back. By default the call waits for the answer. Profiles: explorer \
         (read-only search, answers with findings and file paths), implementer (makes a \
         change and checks it), reviewer (reads and reviews, changes nothing), general."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["run", "start", "status", "result", "cancel", "list"],
                    "description": "run (default): run a sub-agent and wait for its answer; \
                                    start: run it in the background and return its id; \
                                    status: check a background sub-agent; result: wait for one \
                                    and return its answer; cancel: stop one; list: all of them"
                },
                "description": {
                    "type": "string",
                    "description": "Short description of the sub-agent task (3-5 words); for run/start"
                },
                "prompt": {
                    "type": "string",
                    "description": "The full task for the sub-agent, with everything it needs to know; for run/start"
                },
                "profile": {
                    "type": "string",
                    "description": "explorer, implementer, reviewer or general (default), or a configured profile"
                },
                "id": {
                    "type": "string",
                    "description": "Sub-agent id, for status/result/cancel"
                }
            }
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        // A sub-agent may write files (implementer), and its reports are
        // written under .crustly/agents.
        vec![ToolCapability::WriteFiles]
    }

    fn requires_approval(&self) -> bool {
        // The sub-agent's own tool calls go through the approval gate.
        false
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: AgentInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        match action_of(&input) {
            "run" | "start" => {
                for (field, value) in [
                    ("description", &input.description),
                    ("prompt", &input.prompt),
                ] {
                    if is_blank(value) {
                        return Err(ToolError::InvalidInput(format!(
                            "{field} must not be empty"
                        )));
                    }
                }
                Ok(())
            }
            "status" | "result" | "cancel" => parse_id(input.id.as_deref()).map(|_| ()),
            "list" => Ok(()),
            other => Err(ToolError::InvalidInput(format!(
                "unknown action '{other}'; use run, start, status, result, cancel or list"
            ))),
        }
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: AgentInput = serde_json::from_value(input)?;
        let launcher = context.sub_agent_launcher.as_ref().ok_or_else(|| {
            ToolError::Execution(
                "sub-agents are not available here (a sub-agent cannot start another one)"
                    .to_string(),
            )
        })?;

        match action_of(&input) {
            action @ ("run" | "start") => {
                let profile = input
                    .profile
                    .clone()
                    .or_else(|| input.subagent_type.as_deref().map(profile_for_type));
                let task = SubAgentTask {
                    id: Uuid::new_v4(),
                    description: input.description.clone().unwrap_or_default(),
                    prompt: input.prompt.clone().unwrap_or_default(),
                    profile,
                    read_only: context.read_only_mode,
                };
                let id = task.id;
                // A child of the request's token: cancelling the request
                // cancels its sub-agents, while each can still be cancelled
                // on its own.
                let cancel = context.cancel.child_token();
                if action == "run" {
                    let report = launcher.run(task, cancel).await;
                    Ok(report_result(&report))
                } else {
                    launcher.start(task, cancel).await.map_err(|e| {
                        ToolError::Execution(format!("failed to start sub-agent: {e}"))
                    })?;
                    Ok(ToolResult::success(format!(
                        "Started sub-agent {id} in the background. Collect its answer with \
                         agent(action=\"result\", id=\"{id}\")."
                    ))
                    .with_metadata("agent_id".to_string(), id.to_string()))
                }
            }
            action @ ("status" | "result") => {
                let id = parse_id(input.id.as_deref())?;
                match launcher.report(id, action == "result").await {
                    Some(report) => Ok(report_result(&report)),
                    None => Err(ToolError::InvalidInput(format!(
                        "no sub-agent with id {id}"
                    ))),
                }
            }
            "cancel" => {
                let id = parse_id(input.id.as_deref())?;
                Ok(ToolResult::success(if launcher.cancel(id) {
                    format!("Cancelled sub-agent {id}.")
                } else {
                    format!("Sub-agent {id} was not running.")
                }))
            }
            "list" => {
                let reports = launcher.list();
                if reports.is_empty() {
                    return Ok(ToolResult::success(
                        "No sub-agents have been started in this session.".to_string(),
                    ));
                }
                let lines: Vec<String> = reports
                    .iter()
                    .map(|r| {
                        format!(
                            "{} [{}] {} ({})",
                            r.id,
                            status_word(r.status),
                            r.description,
                            r.profile
                        )
                    })
                    .collect();
                Ok(ToolResult::success(lines.join("\n")))
            }
            other => Err(ToolError::InvalidInput(format!("unknown action '{other}'"))),
        }
    }
}

fn action_of(input: &AgentInput) -> &str {
    input.action.as_deref().unwrap_or("run")
}

fn parse_id(id: Option<&str>) -> Result<Uuid> {
    let id = id.ok_or_else(|| ToolError::InvalidInput("id is required".to_string()))?;
    Uuid::parse_str(id.trim())
        .map_err(|_| ToolError::InvalidInput(format!("'{id}' is not a sub-agent id")))
}

/// The profile matching an older `subagent_type` hint.
fn profile_for_type(subagent_type: &str) -> String {
    match subagent_type.to_ascii_lowercase().as_str() {
        "explore" | "plan" => "explorer",
        "verification" => "reviewer",
        _ => "general",
    }
    .to_string()
}

fn status_word(status: SubAgentStatus) -> &'static str {
    match status {
        SubAgentStatus::Queued => "queued",
        SubAgentStatus::Running => "running",
        SubAgentStatus::Completed => "completed",
        SubAgentStatus::Failed => "failed",
        SubAgentStatus::Cancelled => "cancelled",
    }
}

/// A report as the parent model reads it: the answer first, then how it
/// was produced. A failed or cancelled sub-agent is a failed tool call, so
/// the parent does not mistake it for an answer.
fn report_result(report: &SubAgentReport) -> ToolResult {
    let mut text = format!(
        "Sub-agent \"{}\" ({}, id {}) {}.",
        report.description,
        report.profile,
        report.id,
        status_word(report.status)
    );
    if let Some(answer) = &report.answer {
        text.push_str("\n\n");
        text.push_str(answer);
    }
    if let Some(tools) = &report.tools_used {
        text.push_str(&format!("\n\n(Its tool calls: {tools})"));
    }
    let result = match report.status {
        SubAgentStatus::Completed | SubAgentStatus::Queued | SubAgentStatus::Running => {
            ToolResult::success(text)
        }
        SubAgentStatus::Failed | SubAgentStatus::Cancelled => ToolResult {
            success: false,
            output: String::new(),
            error: Some(match &report.error {
                Some(error) => format!("{text}\n{error}"),
                None => text,
            }),
            metadata: Default::default(),
        },
    };
    result.with_metadata("agent_id".to_string(), report.id.to_string())
}

fn is_blank(value: &Option<String>) -> bool {
    value.as_deref().map_or(true, |s| s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

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
        let result = tool
            .validate_input(&serde_json::json!({ "description": "search code", "prompt": "  " }));
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

    #[test]
    fn validate_checks_each_action_s_own_fields() {
        let tool = AgentTool;
        assert!(tool
            .validate_input(&serde_json::json!({ "action": "list" }))
            .is_ok());
        assert!(tool
            .validate_input(&serde_json::json!({ "action": "result" }))
            .is_err());
        assert!(tool
            .validate_input(&serde_json::json!({ "action": "result", "id": "nope" }))
            .is_err());
        assert!(tool
            .validate_input(&serde_json::json!({
                "action": "cancel",
                "id": Uuid::new_v4().to_string()
            }))
            .is_ok());
        assert!(tool
            .validate_input(&serde_json::json!({ "action": "fly" }))
            .is_err());
    }

    #[test]
    fn older_type_hints_map_to_profiles() {
        assert_eq!(profile_for_type("Explore"), "explorer");
        assert_eq!(profile_for_type("Plan"), "explorer");
        assert_eq!(profile_for_type("Verification"), "reviewer");
        assert_eq!(profile_for_type("general-purpose"), "general");
    }

    /// Launcher that records tasks and answers every run with a report.
    #[derive(Debug, Default)]
    struct FakeLauncher {
        tasks: Mutex<Vec<SubAgentTask>>,
    }

    #[async_trait]
    impl crate::llm::tools::SubAgentLauncher for FakeLauncher {
        async fn run(
            &self,
            task: SubAgentTask,
            _cancel: tokio_util::sync::CancellationToken,
        ) -> SubAgentReport {
            self.tasks.lock().unwrap().push(task.clone());
            SubAgentReport {
                id: task.id,
                description: task.description,
                profile: task.profile.unwrap_or_else(|| "general".to_string()),
                status: SubAgentStatus::Completed,
                answer: Some("found it in src/lib.rs".to_string()),
                error: None,
                tools_used: Some("grep ToolRegistry".to_string()),
            }
        }
        async fn start(
            &self,
            task: SubAgentTask,
            _cancel: tokio_util::sync::CancellationToken,
        ) -> std::result::Result<(), String> {
            self.tasks.lock().unwrap().push(task);
            Ok(())
        }
        async fn report(&self, _id: Uuid, _wait: bool) -> Option<SubAgentReport> {
            None
        }
        fn cancel(&self, _id: Uuid) -> bool {
            false
        }
        fn list(&self) -> Vec<SubAgentReport> {
            Vec::new()
        }
        fn profiles(&self) -> Vec<String> {
            vec!["explorer".to_string()]
        }
    }

    #[tokio::test]
    async fn run_waits_for_the_answer_and_returns_it() {
        let launcher = std::sync::Arc::new(FakeLauncher::default());
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_read_only_mode(true)
            .with_sub_agent_launcher(launcher.clone());

        let result = AgentTool
            .execute(
                serde_json::json!({
                    "description": "find usages",
                    "prompt": "Where is ToolRegistry used?",
                    "subagent_type": "Explore"
                }),
                &context,
            )
            .await
            .unwrap();

        assert!(result.success);
        assert!(
            result.output.contains("found it in src/lib.rs"),
            "{}",
            result.output
        );
        assert!(result
            .output
            .contains("(Its tool calls: grep ToolRegistry)"));
        let task = launcher.tasks.lock().unwrap()[0].clone();
        assert_eq!(task.profile.as_deref(), Some("explorer"));
        assert!(
            task.read_only,
            "a plan-mode parent gets a read-only sub-agent"
        );
    }

    #[tokio::test]
    async fn without_a_launcher_the_tool_says_so() {
        let context = ToolExecutionContext::new(Uuid::new_v4());
        let err = AgentTool
            .execute(
                serde_json::json!({"description": "x", "prompt": "y"}),
                &context,
            )
            .await
            .unwrap_err();
        assert!(err.to_string().contains("not available"), "{err}");
    }

    #[test]
    fn a_failed_report_is_a_failed_tool_call() {
        let report = SubAgentReport {
            id: Uuid::new_v4(),
            description: "fix bug".to_string(),
            profile: "implementer".to_string(),
            status: SubAgentStatus::Failed,
            answer: None,
            error: Some("Provider error: timeout".to_string()),
            tools_used: None,
        };
        let result = report_result(&report);
        assert!(!result.success);
        let error = result.error.unwrap();
        assert!(error.contains("failed"));
        assert!(error.contains("timeout"));
    }
}
