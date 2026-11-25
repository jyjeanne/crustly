---
type: Rust Function
title: fallback_to
resource: src/llm/agent/service.rs#L3909-L3915
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/src/llm/agent/service/errors_that_a_retry_cannot_fix_do_not_fall_back
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fallback_to(provider: Arc<ScriptedProvider>, on_overflow: bool) -> Fallback`

# Called by

- [a_failing_provider_falls_back_and_says_so](../../../../../functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so.md)
- [an_overflow_falls_back_only_when_allowed](../../../../../functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)
- [errors_that_a_retry_cannot_fix_do_not_fall_back](../../../../../functions/src/llm/agent/service/errors_that_a_retry_cannot_fix_do_not_fall_back.md)