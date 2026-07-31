---
type: Rust Function
title: message_has_tool_result
resource: src/llm/agent/compaction.rs#L126-L131
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/compaction/compact
---

# Signature

`fn message_has_tool_result(msg: &crate::llm::provider::types::Message) -> bool`

# Called by

- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)