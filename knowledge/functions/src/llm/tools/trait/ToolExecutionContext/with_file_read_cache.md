---
type: Rust Method
title: with_file_read_cache
resource: src/llm/tools/trait.rs#L114-L117
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`pub fn with_file_read_cache(mut self, cache: Arc<FileReadCache>) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)