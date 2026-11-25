---
type: Rust Method
title: complete
resource: src/llm/agent/service.rs#L2528-L2535
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn complete(&self, request: LLMRequest) -> crate::llm::provider::Result<LLMResponse>`

# Calls

- [text_reply](../../../../../../../functions/src/llm/agent/service/text_reply.md)