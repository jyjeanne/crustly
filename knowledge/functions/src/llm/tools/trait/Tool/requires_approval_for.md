---
type: Rust Method
title: requires_approval_for
resource: src/llm/tools/trait.rs#L289-L291
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/forget_memory/the_preview_shows_each_memory_before_anything_is_deleted
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn requires_approval_for(&self, _input: &Value) -> bool`

# Called by

- [the_preview_shows_each_memory_before_anything_is_deleted](../../../../../../functions/src/llm/tools/forget_memory/the_preview_shows_each_memory_before_anything_is_deleted.md)