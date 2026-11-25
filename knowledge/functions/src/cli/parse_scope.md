---
type: Rust Function
title: parse_scope
resource: src/cli/mod.rs#L1118-L1126
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/scope_flags_accept_three_values_and_refuse_others
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_scope(text: &str) -> Result<crate::memory::SearchScope>`

# Called by

- [cmd_memory](../../../functions/src/cli/cmd_memory.md)
- [scope_flags_accept_three_values_and_refuse_others](../../../functions/src/cli/scope_flags_accept_three_values_and_refuse_others.md)