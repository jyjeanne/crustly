//! The agent loop of one request.
//!
//! [`AgentRun`] holds what lives for one request - the context, the tools
//! sent, the middleware pipeline, token counts - and runs rounds until the
//! model answers: call the model, let the middleware decide, run the tools,
//! feed the results back. The rules that stop or redirect the loop (loop
//! guards, limits, verification) live in `middleware`; running the tools of
//! a round lives in `tool_round`.

use super::tool_round::ToolUse;
use super::{AgentService, RequestTools, MAX_OVERFLOW_RETRIES};
use crate::llm::agent::budget::ContextBudget;
use crate::llm::agent::context::AgentContext;
use crate::llm::agent::error::{AgentError, Result};
use crate::llm::agent::middleware::{Decision, Pipeline, RunState, ToolCallRecord, ToolCallView};
use crate::llm::agent::tool_profile::ToolSelection;
use crate::llm::provider::{ContentBlock, LLMResponse, Message, Role};
use crate::llm::tools::ToolExecutionContext;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Everything one request needs to run its loop.
pub(super) struct AgentRun<'a> {
    pub service: &'a AgentService,
    pub model_name: String,
    pub context: AgentContext,
    pub selection: &'a ToolSelection,
    pub tools: &'a RequestTools,
    pub tool_context: ToolExecutionContext,
    pub chunk_tx: Option<&'a mpsc::UnboundedSender<String>>,
    pub cancel: &'a CancellationToken,
    pub pipeline: Pipeline,
}

/// How a request ended.
pub(super) struct RunOutcome {
    /// The reply that answers the user.
    pub response: LLMResponse,
    pub input_tokens: u32,
    pub output_tokens: u32,
    /// Context figures of the last request sent to the model.
    pub last_budget: Option<ContextBudget>,
    /// Shown to the user under the answer (for example, a limit was hit).
    pub notice: Option<String>,
    /// Every tool call handled during the request, in order.
    pub tool_log: Vec<ToolCallRecord>,
    /// Number of rounds that ran tools.
    pub tool_rounds: usize,
}

impl AgentRun<'_> {
    pub async fn run(mut self) -> Result<RunOutcome> {
        let started = Instant::now();
        let mut input_tokens = 0u32;
        let mut output_tokens = 0u32;
        let mut last_budget = None;
        let mut tool_log = Vec::new();
        let mut tool_rounds = 0usize;
        // Successful mutating tool calls since the previous plan task
        // completion. Backs `plan_completion_rejection`: marking a plan task
        // success=true requires evidence of work; each accepted completion
        // resets the counter so the NEXT task needs fresh evidence.
        let mut mutating_evidence = 0usize;
        let mut round = 0usize;

        loop {
            round += 1;
            self.check_cancelled()?;

            let state = RunState {
                round,
                started,
                tokens_used: input_tokens + output_tokens,
                produced_text: false,
                tool_calls: &[],
            };
            if let Decision::Finish { reason, notice } = self.pipeline.before_round(&state) {
                let response = self
                    .finish(&reason, None, &mut input_tokens, &mut output_tokens)
                    .await?;
                return Ok(RunOutcome {
                    response,
                    input_tokens,
                    output_tokens,
                    last_budget,
                    notice,
                    tool_log,
                    tool_rounds,
                });
            }

            // Free context before sending when too little is left for the
            // answer: a small local window fills up within a few tool rounds.
            self.service
                .make_room(&mut self.context, self.tools, 0)
                .await;

            let (response, budget) = self.call_model().await?;
            self.service.record_calibration(&response, &budget);
            last_budget = Some(budget);
            input_tokens += response.usage.input_tokens;
            output_tokens += response.usage.output_tokens;

            // Accumulate prompt-cache metrics (T065)
            if let Some(ref cm) = response.cache_metrics {
                self.context.accumulated_cache_metrics.read_tokens += cm.read_tokens;
                self.context.accumulated_cache_metrics.creation_tokens += cm.creation_tokens;
            }

            log_blocks(&response);

            // Resolve aliases (`Bash`, `shell`, `run_shell_command`...) to the
            // registered name up front, so the approval gate, the plan checks,
            // parallel classification and the result cache all see the same
            // name `ToolRegistry::execute` will run. An alias used to skip
            // AutoPlan's high-risk check, which matches registered names only.
            let tool_uses: Vec<ToolUse> = response
                .content
                .iter()
                .filter_map(|block| match block {
                    ContentBlock::ToolUse { id, name, input } => Some((
                        id.clone(),
                        self.service.tool_registry.resolve_name(name),
                        input.clone(),
                    )),
                    _ => None,
                })
                .collect();
            tracing::debug!("Found {} tool uses to execute", tool_uses.len());

            let produced_text = response
                .content
                .iter()
                .any(|b| matches!(b, ContentBlock::Text { text } if !text.trim().is_empty()));

            if tool_uses.is_empty() {
                let state = RunState {
                    round,
                    started,
                    tokens_used: input_tokens + output_tokens,
                    produced_text,
                    tool_calls: &[],
                };
                if let Decision::Nudge { message } = self.pipeline.on_final_answer(&state) {
                    tracing::info!("Final answer held back: {}", message);
                    self.context.add_message(Message {
                        role: Role::Assistant,
                        content: response.content.clone(),
                    });
                    self.context.add_message(Message::user(message));
                    continue;
                }
                tracing::debug!("No tool uses found, completing with final response");
                return Ok(RunOutcome {
                    response,
                    input_tokens,
                    output_tokens,
                    last_budget,
                    notice: None,
                    tool_log,
                    tool_rounds,
                });
            }

            let views: Vec<ToolCallView> = tool_uses
                .iter()
                .map(|(_, name, input)| ToolCallView {
                    name: name.clone(),
                    input: input.clone(),
                    capabilities: self
                        .service
                        .tool_registry
                        .get(name)
                        .map(|t| t.capabilities())
                        .unwrap_or_default(),
                })
                .collect();
            let state = RunState {
                round,
                started,
                tokens_used: input_tokens + output_tokens,
                produced_text,
                tool_calls: &views,
            };
            if let Decision::Finish { reason, notice } = self.pipeline.on_tool_calls(&state) {
                let response = self
                    .finish(
                        &reason,
                        Some(response),
                        &mut input_tokens,
                        &mut output_tokens,
                    )
                    .await?;
                return Ok(RunOutcome {
                    response,
                    input_tokens,
                    output_tokens,
                    last_budget,
                    notice,
                    tool_log,
                    tool_rounds,
                });
            }

            let tool_round = self
                .service
                .execute_tool_round(
                    tool_uses,
                    self.selection,
                    self.tools,
                    &self.tool_context,
                    &mut mutating_evidence,
                    round,
                    self.cancel,
                )
                .await;
            tool_rounds += 1;
            self.pipeline.after_tools(&tool_round.records);
            tool_log.extend(tool_round.records);

            self.context.add_message(Message {
                role: Role::Assistant,
                content: response.content.clone(),
            });
            self.context.add_message(Message {
                role: Role::User,
                content: tool_round.results,
            });

            // Compaction is not triggered here: `make_room` at the top of the
            // next round checks the full request - system prompt and tool
            // schemas included - against the window before anything is sent.
        }
    }

    fn check_cancelled(&self) -> Result<()> {
        if self.cancel.is_cancelled() {
            tracing::info!("Request cancelled by the user");
            return Err(AgentError::Cancelled);
        }
        Ok(())
    }

    /// Call the model for one round. Streams when `chunk_tx` is set. When
    /// the provider reports that the prompt overflowed its window, free more
    /// context and retry the same round instead of failing the request.
    async fn call_model(&mut self) -> Result<(LLMResponse, ContextBudget)> {
        let mut overflow_retries = 0;
        loop {
            let budget = self.service.measure_budget(&self.context, self.tools);
            tracing::debug!(
                "Sending {} tool definitions; {} of {} context tokens left for the answer",
                self.tools.defs.len(),
                budget.available(),
                budget.window
            );
            let request = AgentService::build_request(
                &self.model_name,
                &self.context,
                self.tools,
                budget.completion_budget(),
            );
            let call = AgentService::call_provider_streaming(
                &self.service.provider,
                request,
                self.chunk_tx,
                &self.model_name,
            );
            let outcome = tokio::select! {
                outcome = call => outcome,
                _ = self.cancel.cancelled() => return Err(AgentError::Cancelled),
            };
            match outcome {
                Ok(response) => return Ok((response, budget)),
                Err(e) if e.is_context_overflow() && overflow_retries < MAX_OVERFLOW_RETRIES => {
                    overflow_retries += 1;
                    tracing::warn!(
                        "Prompt overflowed the context window ({}); freeing context and \
                         retrying ({}/{})",
                        e,
                        overflow_retries,
                        MAX_OVERFLOW_RETRIES
                    );
                    if !self
                        .service
                        .make_room(&mut self.context, self.tools, overflow_retries)
                        .await
                    {
                        return Err(AgentError::Provider(e));
                    }
                }
                Err(e) => return Err(AgentError::Provider(e)),
            }
        }
    }

    /// Stop calling tools: tell the model `reason` and ask it, with no tools
    /// offered, to answer from what it has.
    ///
    /// `pending` is the reply whose tool calls will not run. Each of them gets
    /// a "not run" result, so the conversation stays valid (providers reject
    /// a tool use without its result), and the instruction follows in its own
    /// user message (the OpenAI adapter drops text that shares a message with
    /// tool results). If the final call fails, the pending reply is the
    /// answer.
    async fn finish(
        &mut self,
        reason: &str,
        pending: Option<LLMResponse>,
        input_tokens: &mut u32,
        output_tokens: &mut u32,
    ) -> Result<LLMResponse> {
        if let Some(pending) = &pending {
            let not_run: Vec<ContentBlock> = pending
                .content
                .iter()
                .filter_map(|block| match block {
                    ContentBlock::ToolUse { id, .. } => Some(ContentBlock::ToolResult {
                        tool_use_id: id.clone(),
                        content: "Not run: the request is being wrapped up.".to_string(),
                        is_error: Some(true),
                    }),
                    _ => None,
                })
                .collect();
            self.context.add_message(Message {
                role: Role::Assistant,
                content: pending.content.clone(),
            });
            if !not_run.is_empty() {
                self.context.add_message(Message {
                    role: Role::User,
                    content: not_run,
                });
            }
        }
        self.context.add_message(Message::user(reason));

        // Ask without tools so the model is forced to produce a text answer.
        let no_tools = RequestTools::new(Vec::new());
        let max_tokens = self
            .service
            .measure_budget(&self.context, &no_tools)
            .completion_budget()
            .min(2048);
        let request =
            AgentService::build_request(&self.model_name, &self.context, &no_tools, max_tokens);
        let outcome = tokio::select! {
            outcome = self.service.provider.complete(request) => outcome,
            _ = self.cancel.cancelled() => return Err(AgentError::Cancelled),
        };
        match (outcome, pending) {
            (Ok(response), _) => {
                *input_tokens += response.usage.input_tokens;
                *output_tokens += response.usage.output_tokens;
                Ok(response)
            }
            (Err(e), Some(pending)) => {
                tracing::warn!("Final answer request failed ({}); using the last reply", e);
                Ok(pending)
            }
            (Err(e), None) => Err(AgentError::Provider(e)),
        }
    }
}

fn log_blocks(response: &LLMResponse) {
    tracing::debug!("Response has {} content blocks", response.content.len());
    for (i, block) in response.content.iter().enumerate() {
        match block {
            ContentBlock::Text { text } => {
                tracing::debug!(
                    "Block {}: Text ({}...)",
                    i,
                    &text.chars().take(50).collect::<String>()
                );
            }
            ContentBlock::ToolUse { id, name, .. } => {
                tracing::debug!("Block {}: ToolUse {{ name: {}, id: {} }}", i, name, id);
            }
            _ => tracing::debug!("Block {}: Other content block", i),
        }
    }
}
