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
use crate::llm::provider::{ContentBlock, LLMResponse, Message, Provider, ProviderError, Role};
use crate::llm::tools::ToolExecutionContext;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Everything one request needs to run its loop.
pub(super) struct AgentRun<'a> {
    pub service: &'a AgentService,
    /// The provider answering this request: the service's own, or its
    /// fallback once the run has fallen back.
    pub provider: Arc<dyn Provider>,
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
    /// Cost of every model call, each priced by the provider that made it.
    pub cost: f64,
    /// The provider that produced the answer.
    pub provider_name: String,
    /// Context figures of the last request sent to the model.
    pub last_budget: Option<ContextBudget>,
    /// Shown to the user under the answer (a limit was hit, the answer came
    /// from the fallback provider).
    pub notice: Option<String>,
    /// Every tool call handled during the request, in order.
    pub tool_log: Vec<ToolCallRecord>,
    /// Number of rounds that ran tools.
    pub tool_rounds: usize,
}

/// What the run has used so far.
#[derive(Default)]
struct Totals {
    input_tokens: u32,
    output_tokens: u32,
    cost: f64,
    last_budget: Option<ContextBudget>,
    tool_log: Vec<ToolCallRecord>,
    tool_rounds: usize,
    /// Set once the run fell back: why, for the notice.
    fallback_reason: Option<String>,
}

impl AgentRun<'_> {
    pub async fn run(mut self) -> Result<RunOutcome> {
        let started = Instant::now();
        let mut totals = Totals::default();
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
                tokens_used: totals.input_tokens + totals.output_tokens,
                produced_text: false,
                tool_calls: &[],
            };
            if let Decision::Finish { reason, notice } = self.pipeline.before_round(&state) {
                let response = self.finish(&reason, None, &mut totals).await?;
                return Ok(self.outcome(response, notice, totals));
            }

            // Free context before sending when too little is left for the
            // answer: a small local window fills up within a few tool rounds.
            self.service
                .make_room(&mut self.context, self.tools, 0)
                .await;

            let (response, budget) = self.call_model(&mut totals).await?;
            // A fallback provider counts tokens its own way: only learn from
            // the provider whose window the budget is about.
            if totals.fallback_reason.is_none() {
                self.service.record_calibration(&response, &budget);
            }
            totals.last_budget = Some(budget);
            self.add_usage(&response, &mut totals);

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
                    tokens_used: totals.input_tokens + totals.output_tokens,
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
                return Ok(self.outcome(response, None, totals));
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
                tokens_used: totals.input_tokens + totals.output_tokens,
                produced_text,
                tool_calls: &views,
            };
            if let Decision::Finish { reason, notice } = self.pipeline.on_tool_calls(&state) {
                let response = self.finish(&reason, Some(response), &mut totals).await?;
                return Ok(self.outcome(response, notice, totals));
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
            totals.tool_rounds += 1;
            self.pipeline.after_tools(&tool_round.records);
            totals.tool_log.extend(tool_round.records);

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

    fn outcome(&self, response: LLMResponse, notice: Option<String>, totals: Totals) -> RunOutcome {
        let fallback_notice = totals.fallback_reason.as_ref().map(|reason| {
            format!(
                "_☁ Answered by the fallback provider {} ({}): {}._",
                self.provider.name(),
                response.model,
                reason
            )
        });
        let notice = match (notice, fallback_notice) {
            (Some(a), Some(b)) => Some(format!("{a}\n\n{b}")),
            (a, b) => a.or(b),
        };
        RunOutcome {
            response,
            input_tokens: totals.input_tokens,
            output_tokens: totals.output_tokens,
            cost: totals.cost,
            provider_name: self.provider.name().to_string(),
            last_budget: totals.last_budget,
            notice,
            tool_log: totals.tool_log,
            tool_rounds: totals.tool_rounds,
        }
    }

    fn add_usage(&self, response: &LLMResponse, totals: &mut Totals) {
        totals.input_tokens += response.usage.input_tokens;
        totals.output_tokens += response.usage.output_tokens;
        totals.cost += self.provider.calculate_cost(
            &response.model,
            response.usage.input_tokens,
            response.usage.output_tokens,
        );
    }

    fn check_cancelled(&self) -> Result<()> {
        if self.cancel.is_cancelled() {
            tracing::info!("Request cancelled by the user");
            return Err(AgentError::Cancelled);
        }
        Ok(())
    }

    /// Call the model for one round. Streams when `chunk_tx` is set.
    ///
    /// When the provider reports that the prompt overflowed its window, free
    /// more context and retry the same round instead of failing. When that
    /// cannot free enough, or the provider is unreachable or failing, fall
    /// back to the configured fallback provider for the rest of the request.
    async fn call_model(&mut self, totals: &mut Totals) -> Result<(LLMResponse, ContextBudget)> {
        let mut overflow_retries = 0;
        loop {
            let budget = self.service.measure_budget(&self.context, self.tools);
            if budget.available() == 0 {
                // `completion_budget()` must never be zero because providers
                // reject it, but sending one token when the prompt already
                // fills the window guarantees an oversized request. `run`
                // has already tried to make room before entering this method;
                // at this point either use a larger fallback or fail locally.
                let overflow =
                    ProviderError::ContextLengthExceeded(budget.estimated_prompt_tokens());
                if self.fall_back(true, &overflow, totals) {
                    continue;
                }
                return Err(AgentError::ContextTooLarge {
                    current: budget.estimated_prompt_tokens() as usize,
                    limit: budget.window as usize,
                });
            }
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
                &self.provider,
                request,
                self.chunk_tx,
                &self.model_name,
            );
            let outcome = tokio::select! {
                outcome = call => outcome,
                _ = self.cancel.cancelled() => return Err(AgentError::Cancelled),
            };
            let error = match outcome {
                Ok(response) => return Ok((response, budget)),
                Err(e) => e,
            };

            if error.is_context_overflow() {
                if overflow_retries < MAX_OVERFLOW_RETRIES {
                    overflow_retries += 1;
                    tracing::warn!(
                        "Prompt overflowed the context window ({}); freeing context and \
                         retrying ({}/{})",
                        error,
                        overflow_retries,
                        MAX_OVERFLOW_RETRIES
                    );
                    if self
                        .service
                        .make_room(&mut self.context, self.tools, overflow_retries)
                        .await
                    {
                        continue;
                    }
                }
                if self.fall_back(true, &error, totals) {
                    continue;
                }
            } else if error.is_retryable() && self.fall_back(false, &error, totals) {
                continue;
            }
            return Err(AgentError::Provider(error));
        }
    }

    /// Switch the rest of the request to the fallback provider, when one is
    /// configured for this kind of failure and the run has not already
    /// fallen back. Returns whether it did.
    fn fall_back(&mut self, overflow: bool, error: &ProviderError, totals: &mut Totals) -> bool {
        let Some(fallback) = &self.service.fallback else {
            return false;
        };
        let allowed = if overflow {
            fallback.on_overflow
        } else {
            fallback.on_error
        };
        if !allowed || totals.fallback_reason.is_some() {
            return false;
        }

        let reason = if overflow {
            format!(
                "the prompt did not fit {}'s context window",
                self.provider.name()
            )
        } else {
            format!("{} failed ({})", self.provider.name(), error)
        };
        // The fallback answers with its own model, never the local model's
        // name, and budgets against its own (usually much larger) window.
        let model = fallback.provider.default_model().to_string();
        tracing::warn!(
            "[FAILOVER] {}; sending the rest of this request to {} ({})",
            reason,
            fallback.provider.name(),
            model
        );
        if let Some(window) = fallback.provider.context_window(&model) {
            self.context.max_tokens = self.context.max_tokens.max(window as usize);
        }
        self.provider = fallback.provider.clone();
        self.model_name = model;
        totals.fallback_reason = Some(reason);
        true
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
        totals: &mut Totals,
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
            outcome = self.provider.complete(request) => outcome,
            _ = self.cancel.cancelled() => return Err(AgentError::Cancelled),
        };
        match (outcome, pending) {
            (Ok(response), _) => {
                self.add_usage(&response, totals);
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
