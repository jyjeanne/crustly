---
type: Rust Method
title: complete
resource: src/llm/agent/service.rs#L3794-L3800
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn complete( &self, _request: LLMRequest, ) -> crate::llm::provider::Result<LLMResponse>`

# Calls

- [forget](../../../../../../../functions/src/memory/MemoryStore/forget.md)
- [text_reply](../../../../../../../functions/src/llm/agent/service/text_reply.md)