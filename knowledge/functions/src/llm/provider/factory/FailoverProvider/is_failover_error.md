---
type: Rust Method
title: is_failover_error
resource: src/llm/provider/factory.rs#L56-L63
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

`fn is_failover_error(err: &ProviderError) -> bool`

# Called by

- [complete](../../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/stream.md)