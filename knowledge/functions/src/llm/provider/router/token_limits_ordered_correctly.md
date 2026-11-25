---
type: Rust Function
title: token_limits_ordered_correctly
resource: src/llm/provider/router.rs#L123-L135
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/router/ModelRouter/default_for_test
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/router/ModelRouter/max_output_tokens
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn token_limits_ordered_correctly()`

# Calls

- [default_for_test](../../../../../functions/src/llm/provider/router/ModelRouter/default_for_test.md)
- [max_output_tokens](../../../../../functions/src/llm/provider/router/ModelRouter/max_output_tokens.md)