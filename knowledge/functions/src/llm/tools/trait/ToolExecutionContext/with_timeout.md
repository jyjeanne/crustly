---
type: Rust Method
title: with_timeout
resource: src/llm/tools/trait.rs#L170-L173
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/bash/test_bash_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/test_execution_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_timeout(mut self, timeout_secs: u64) -> Self`

# Called by

- [test_bash_timeout](../../../../../../functions/src/llm/tools/bash/test_bash_timeout.md)
- [test_bash_timeout_field_overrides_context_default](../../../../../../functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default.md)
- [test_execution_context](../../../../../../functions/src/llm/tools/trait/test_execution_context.md)