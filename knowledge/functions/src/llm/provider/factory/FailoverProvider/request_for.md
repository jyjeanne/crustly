---
type: Rust Method
title: request_for
resource: src/llm/provider/factory.rs#L44-L54
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/FailoverProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/FailoverProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn request_for( &self, index: usize, request: &super::types::LLMRequest, ) -> super::types::LLMRequest`

# Called by

- [complete](../../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/stream.md)