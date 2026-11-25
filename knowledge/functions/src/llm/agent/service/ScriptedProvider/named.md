---
type: Rust Method
title: named
resource: src/llm/agent/service.rs#L2515-L2519
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn named(mut self, name: &'static str, model: &'static str) -> Self`

# Called by

- [a_failing_provider_falls_back_and_says_so](../../../../../../functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so.md)
- [an_overflow_falls_back_only_when_allowed](../../../../../../functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)