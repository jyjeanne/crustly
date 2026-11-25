---
type: Rust Method
title: send_message
resource: src/tui/app.rs#L2376-L2476
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/send_message_still_works_for_a_different_session_than_the_one_processing
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn send_message(&mut self, content: String) -> Result<()>`

# Calls

- [exposed_tool_names](../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_names.md)
- [analyze_and_transform_for](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [send_message_with_tools_and_mode_streaming](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)
- [send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight](../../../../../functions/src/tui/app/send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight.md)
- [send_message_still_works_for_a_different_session_than_the_one_processing](../../../../../functions/src/tui/app/send_message_still_works_for_a_different_session_than_the_one_processing.md)