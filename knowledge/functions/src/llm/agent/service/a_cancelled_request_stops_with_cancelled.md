---
type: Rust Function
title: a_cancelled_request_stops_with_cancelled
resource: src/llm/agent/service.rs#L3222-L3244
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/session_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_cancelled_request_stops_with_cancelled()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [send_message_with_tools_and_mode_streaming](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)