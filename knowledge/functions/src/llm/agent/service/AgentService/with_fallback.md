---
type: Rust Method
title: with_fallback
resource: src/llm/agent/service.rs#L606-L609
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_fallback(mut self, fallback: Fallback) -> Self`

# Called by

- [configure_agent](../../../../../../functions/src/cli/configure_agent.md)
- [a_failing_provider_falls_back_and_says_so](../../../../../../functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so.md)
- [an_overflow_falls_back_only_when_allowed](../../../../../../functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)
- [errors_that_a_retry_cannot_fix_do_not_fall_back](../../../../../../functions/src/llm/agent/service/errors_that_a_retry_cannot_fix_do_not_fall_back.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)