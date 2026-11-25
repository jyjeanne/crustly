---
type: Rust Method
title: default_anthropic
resource: src/llm/provider/router.rs#L77-L86
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/router/ModelRouter/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/router/ModelRouter/default/default
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn default_anthropic() -> Self`

# Calls

- [new](../../../../../../functions/src/llm/provider/router/ModelRouter/new.md)

# Called by

- [default](../../../../../../functions/src/llm/provider/router/ModelRouter/default/default.md)