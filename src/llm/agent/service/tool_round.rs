//! One round of tool execution: the tool calls of a single model reply, run
//! through the same stages every time.
//!
//! Each call goes: exposed by the tool profile? -> arguments plausible for
//! the schema? -> (read-only calls: cached result, else run concurrently) or
//! (others, one at a time: plan-evidence gate -> approval -> run). Every
//! call produces exactly one `ToolResult` block, so the reply's tool uses
//! stay paired with results whatever happens, and one [`ToolCallRecord`] for
//! the middleware and the tool log.

use super::{
    has_mutating_capability, is_parallelizable, plan_completion_rejection, AgentService,
    RequestTools, ToolApprovalInfo,
};
use crate::llm::agent::middleware::ToolCallRecord;
use crate::llm::agent::tool_profile::ToolSelection;
use crate::llm::provider::ContentBlock;
use crate::llm::tools::cache::CacheKey;
use crate::llm::tools::schema_check::{check_arguments, describe_arguments};
use crate::llm::tools::{ToolCapability, ToolError, ToolExecutionContext, ToolResult};
use futures::future::join_all;
use serde_json::Value;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// A tool call from the model: id, registered (canonical) name, arguments.
pub(super) type ToolUse = (String, String, Value);

/// What one round of tool execution produced.
pub(super) struct ToolRound {
    /// One `ToolResult` block per call, to send back to the model.
    pub results: Vec<ContentBlock>,
    /// One record per call, in execution order.
    pub records: Vec<ToolCallRecord>,
}

impl ToolRound {
    fn push(&mut self, (block, record): (ContentBlock, ToolCallRecord)) {
        self.results.push(block);
        self.records.push(record);
    }

    /// A call answered without running the tool.
    fn refuse(&mut self, tool_id: String, name: String, input: Value, message: String) {
        self.push(refused(tool_id, name, input, message));
    }
}

fn refused(
    tool_id: String,
    name: String,
    input: Value,
    message: String,
) -> (ContentBlock, ToolCallRecord) {
    (
        ContentBlock::ToolResult {
            tool_use_id: tool_id,
            content: message,
            is_error: Some(true),
        },
        ToolCallRecord {
            name,
            input,
            success: false,
        },
    )
}

/// Turn a tool's outcome into its result block and record. When the tool
/// rejected its arguments, the expected arguments are appended so the model
/// can correct the call.
fn completed(
    tool_id: String,
    name: String,
    input: Value,
    outcome: std::result::Result<ToolResult, ToolError>,
    schema: Option<&Value>,
) -> (ContentBlock, ToolCallRecord) {
    let (content, success) = match outcome {
        Ok(result) if result.success => (result.output, true),
        Ok(result) => (
            result
                .error
                .unwrap_or_else(|| "Tool execution failed".to_string()),
            false,
        ),
        Err(e @ (ToolError::InvalidInput(_) | ToolError::Json(_))) => {
            let mut message = format!("Tool execution error: {e}");
            if let Some(schema) = schema {
                message.push_str(&format!(
                    "\nExpected arguments for {name}: {}",
                    describe_arguments(schema)
                ));
            }
            (message, false)
        }
        Err(e) => (format!("Tool execution error: {e}"), false),
    };
    (
        ContentBlock::ToolResult {
            tool_use_id: tool_id,
            content,
            is_error: Some(!success),
        },
        ToolCallRecord {
            name,
            input,
            success,
        },
    )
}

impl AgentService {
    /// Run the tool calls of one model reply.
    ///
    /// `mutating_evidence` counts successful mutating calls since the last
    /// accepted plan-task completion (see `plan_completion_rejection`).
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn execute_tool_round(
        &self,
        tool_uses: Vec<ToolUse>,
        selection: &ToolSelection,
        tools: &RequestTools,
        tool_context: &ToolExecutionContext,
        mutating_evidence: &mut usize,
        round: usize,
        cancel: &CancellationToken,
    ) -> ToolRound {
        let mut out = ToolRound {
            results: Vec::new(),
            records: Vec::new(),
        };

        // A tool the profile does not expose gets an error result instead
        // of running: the model only learned its name some other way (an
        // earlier turn, a hallucination), and the core profile exists to
        // keep it out of the conversation.
        let (tool_uses, hidden): (Vec<_>, Vec<_>) = tool_uses
            .into_iter()
            .partition(|(_, name, _)| selection.exposes(name));
        for (tool_id, name, input) in hidden {
            tracing::warn!(
                "Model called '{}', which the {} tool profile does not expose",
                name,
                selection.label()
            );
            let available: Vec<&str> = tools.defs.iter().map(|t| t.name.as_str()).collect();
            let message = format!(
                "Tool '{}' is not available in this session. Use one of: {}.",
                name,
                available.join(", ")
            );
            out.refuse(tool_id, name, input, message);
        }

        // Arguments that cannot match the tool's schema are refused with the
        // expected arguments, instead of failing inside the tool with a
        // deserialization error a small model cannot act on.
        let mut runnable = Vec::with_capacity(tool_uses.len());
        for (tool_id, name, input) in tool_uses {
            let schema = self.tool_registry.get(&name).map(|t| t.input_schema());
            if let Some(schema) = &schema {
                if let Err(problem) = check_arguments(&input, schema) {
                    tracing::warn!("Refused '{}' call with bad arguments: {}", name, problem);
                    let message = format!(
                        "Invalid arguments for {name}: {problem}.\nExpected arguments: {}",
                        describe_arguments(schema)
                    );
                    out.refuse(tool_id, name, input, message);
                    continue;
                }
            }
            runnable.push((tool_id, name, input));
        }

        // Read-only, idempotent tools run concurrently; the rest one at a time.
        let (parallel, sequential): (Vec<_>, Vec<_>) = runnable
            .into_iter()
            .partition(|(_, name, _)| is_parallelizable(name));

        let parallel_futures: Vec<_> = parallel
            .into_iter()
            .map(|(tool_id, name, input)| {
                let registry = Arc::clone(&self.tool_registry);
                let cache = Arc::clone(&self.tool_cache);
                let ctx = tool_context.clone();
                async move {
                    // Results are cached per session to avoid redundant calls
                    // (T037 / T049). A cached result answers the call without
                    // running the tool, so check the permission policy here as
                    // well: every call faces the policy, not only the ones
                    // that reach `ToolRegistry::execute`.
                    let cache_key = CacheKey::from_tool(&name, &input);
                    if !registry.is_denied(&name, &input) {
                        if let Some(cached) = cache.get(&cache_key) {
                            tracing::debug!("Cache hit for tool '{}'", name);
                            return completed(
                                tool_id,
                                name,
                                input,
                                Ok(ToolResult::success(cached)),
                                None,
                            );
                        }
                    }
                    let schema = registry.get(&name).map(|t| t.input_schema());
                    let outcome = registry.execute(&name, input.clone(), &ctx).await;
                    if let Ok(result) = &outcome {
                        if result.success {
                            cache.insert_for_tool(cache_key, result.output.clone());
                        }
                    }
                    completed(tool_id, name, input, outcome, schema.as_ref())
                }
            })
            .collect();
        for pair in join_all(parallel_futures).await {
            out.push(pair);
        }

        // Set when a tool with a mutating capability actually executes
        // (approved or auto), so read caches can be invalidated below.
        let mut mutating_tool_ran = false;
        let mut sequential = sequential.into_iter();
        while let Some((tool_id, name, input)) = sequential.next() {
            tracing::info!(
                "Executing tool '{}' (iteration {}/{})",
                name,
                round,
                self.max_tool_iterations
            );

            // A plan task may only be marked success=true with evidence
            // that its work happened (see plan_completion_rejection).
            if name == "plan" {
                if let Some(reason) = plan_completion_rejection(
                    &input,
                    *mutating_evidence,
                    &tool_context.working_directory,
                    tool_context.session_id,
                ) {
                    tracing::warn!(
                        "⚠️ Blocked a plan complete_task without evidence of work \
                         (mutating calls since last completion: {})",
                        mutating_evidence
                    );
                    out.refuse(tool_id, name, input, reason);
                    continue;
                }
            }

            let Some(tool) = self.tool_registry.get(&name) else {
                let outcome = Err(ToolError::NotFound(name.clone()));
                out.push(completed(tool_id, name, input, outcome, None));
                continue;
            };

            // Check if approval is needed. `requires_approval` is a static
            // property of the tool (bash always returns true), so it alone
            // would re-prompt for every `ls`. Defer to the permission policy
            // too: if it explicitly vouches for this exact command - the
            // program is on `security.allow_bash` and there is no active
            // shell operator to smuggle another one in - that stands in for
            // the user's approval. Anything not allowlisted still prompts.
            let needs_approval = tool.requires_approval()
                && !self.auto_approve_tools
                && !tool_context.auto_approve
                && !self.tool_registry.is_trusted(&name, &input);

            let mut run_context = tool_context.clone();
            if needs_approval {
                let Some(approval_callback) = &self.approval_callback else {
                    tracing::warn!(
                        "Tool '{}' requires approval but no approval callback configured",
                        name
                    );
                    out.refuse(
                        tool_id,
                        name,
                        input,
                        "Tool requires approval but no approval mechanism configured".to_string(),
                    );
                    continue;
                };

                let info = ToolApprovalInfo {
                    tool_name: name.clone(),
                    tool_description: tool.description().to_string(),
                    tool_input: input.clone(),
                    capabilities: tool
                        .capabilities()
                        .iter()
                        .map(|c| format!("{:?}", c))
                        .collect(),
                };
                // Log the inputs: for `bash` the command decides whether the
                // allowlist trusts it, and without it a prompt here is
                // impossible to explain.
                tracing::info!(
                    "Requesting user approval for tool '{}' with input {}",
                    name,
                    input
                );
                let answer = tokio::select! {
                    answer = approval_callback(info) => answer,
                    _ = cancel.cancelled() => {
                        // Answer this call and every remaining one so each
                        // tool use still has its result.
                        let message = "Not run: the user cancelled the request".to_string();
                        out.refuse(tool_id, name, input, message.clone());
                        for (tool_id, name, input) in sequential.by_ref() {
                            out.refuse(tool_id, name, input, message.clone());
                        }
                        break;
                    }
                };
                match answer {
                    Ok(true) => {
                        tracing::info!("User approved tool '{}'", name);
                        // The user approved this execution.
                        run_context.auto_approve = true;
                    }
                    Ok(false) => {
                        tracing::warn!("User denied approval for tool '{}'", name);
                        out.refuse(
                            tool_id,
                            name,
                            input,
                            "User denied permission to execute this tool".to_string(),
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::error!("Approval callback error: {}", e);
                        out.refuse(
                            tool_id,
                            name,
                            input,
                            format!("Approval request failed: {}", e),
                        );
                        continue;
                    }
                }
            }

            let is_mutating = has_mutating_capability(&tool.capabilities());
            mutating_tool_ran |= is_mutating;
            let completes_plan_task = name == "plan"
                && input.get("operation").and_then(|v| v.as_str()) == Some("complete_task");
            let schema = tool.input_schema();
            let outcome = self
                .tool_registry
                .execute(&name, input.clone(), &run_context)
                .await;
            if matches!(&outcome, Ok(result) if result.success) {
                if is_mutating {
                    *mutating_evidence += 1;
                }
                // Checkpoint: an accepted completion consumes the evidence,
                // so the NEXT task needs its own.
                if completes_plan_task {
                    *mutating_evidence = 0;
                }
            }
            out.push(completed(tool_id, name, input, outcome, Some(&schema)));
        }

        // A mutating tool (write_file, edit_file, bash, …) may have changed
        // the filesystem — drop cached results of every tool that reads it,
        // so the next read sees the current state instead of a stale entry
        // within its TTL. Which tools count as reads comes from their
        // declared capabilities, not a hardcoded name list; unknown tools are
        // invalidated too.
        if mutating_tool_ran {
            self.tool_cache.invalidate_matching(|tool_name| {
                self.tool_registry.get(tool_name).map_or(true, |t| {
                    t.capabilities().contains(&ToolCapability::ReadFiles)
                })
            });
        }

        out
    }
}
