---
type: Rust Method
title: with_timeout
resource: src/llm/tools/trait.rs#L94-L97
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/bash/test_bash_timeout
  - functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default
  - functions/src/llm/tools/trait/test_execution_context
---

# Signature

`pub fn with_timeout(mut self, timeout_secs: u64) -> Self`

# Called by

- [test_bash_timeout](../../../../../../functions/src/llm/tools/bash/test_bash_timeout.md)
- [test_bash_timeout_field_overrides_context_default](../../../../../../functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default.md)
- [test_execution_context](../../../../../../functions/src/llm/tools/trait/test_execution_context.md)