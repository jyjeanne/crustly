---
type: Rust Method
title: send_message_with_tools_inner
resource: src/llm/agent/service.rs#L1303-L1512
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/from_db_messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/tool_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/RequestTools/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/system_prompt_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/augment_message_with_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/user_turn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_file_read_cache
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/child_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/final_text_and_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/summarize_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_tool_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/log_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/is_core_only
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_cancellable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn send_message_with_tools_inner( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, chunk_tx: Option<mpsc::UnboundedSender<String>>, cancel: CancellationToken, ) -> Result<AgentResponse>`

# Calls

- [new](../../../../../../functions/src/services/session/SessionService/new.md)
- [get_session](../../../../../../functions/src/services/session/SessionService/get_session.md)
- [new](../../../../../../functions/src/services/message/MessageService/new.md)
- [list_messages_for_session](../../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [new](../../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)
- [classify_tier](../../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier.md)
- [from_db_messages](../../../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [tool_selection](../../../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [new](../../../../../../functions/src/llm/agent/service/RequestTools/new.md)
- [exposed_tool_definitions](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_definitions.md)
- [system_prompt_for](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
- [next](../../../../../../functions/src/tui/events/EventHandler/next.md)
- [augment_message_with_pdf](../../../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [memory_block](../../../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user_turn](../../../../../../functions/src/llm/agent/service/user_turn.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [new](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_auto_approve](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)
- [with_working_directory](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)
- [with_read_only_mode](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)
- [with_file_read_cache](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_file_read_cache.md)
- [with_cancel](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_cancel.md)
- [new](../../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/new.md)
- [child_template](../../../../../../functions/src/llm/agent/service/AgentService/child_template.md)
- [with_sub_agent_launcher](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)
- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)
- [summarize_tool_calls](../../../../../../functions/src/llm/agent/project_memory/summarize_tool_calls.md)
- [update_message_tool_log](../../../../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [log_progress](../../../../../../functions/src/llm/agent/service/AgentService/log_progress.md)
- [update_message_usage](../../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_metrics](../../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [update_session_usage](../../../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [usage](../../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [is_core_only](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)

# Called by

- [send_message_with_tools_and_mode](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode.md)
- [send_message_with_tools_cancellable](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_cancellable.md)
- [send_message_with_tools_and_mode_streaming](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming.md)