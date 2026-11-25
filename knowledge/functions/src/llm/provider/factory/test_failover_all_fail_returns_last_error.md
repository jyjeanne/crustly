---
type: Rust Function
title: test_failover_all_fail_returns_last_error
resource: src/llm/provider/factory.rs#L754-L766
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/FailoverProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/FailoverProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_failover_all_fail_returns_last_error()`

# Calls

- [new](../../../../../functions/src/llm/provider/factory/FailoverProvider/new.md)
- [complete](../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/complete.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)