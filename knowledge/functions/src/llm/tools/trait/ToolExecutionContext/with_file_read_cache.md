---
type: Rust Method
title: with_file_read_cache
resource: src/llm/tools/trait.rs#L190-L193
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_file_read_cache(mut self, cache: Arc<FileReadCache>) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)