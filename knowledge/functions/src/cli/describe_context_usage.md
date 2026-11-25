---
type: Rust Function
title: describe_context_usage
resource: src/cli/mod.rs#L2178-L2191
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn describe_context_usage(usage: &crate::llm::agent::ContextUsage) -> String`

# Called by

- [cmd_run](../../../functions/src/cli/cmd_run.md)