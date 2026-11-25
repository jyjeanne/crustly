---
type: Rust Function
title: tool_call_signature
resource: src/llm/agent/middleware.rs#L165-L234
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/signature_uses_path_key_so_different_edits_do_not_collide
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/signature_accepts_file_path_alias
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn tool_call_signature(name: &str, input: &Value) -> String`

# Called by

- [on_tool_calls](../../../../../functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls.md)
- [signature_uses_path_key_so_different_edits_do_not_collide](../../../../../functions/src/llm/agent/service/signature_uses_path_key_so_different_edits_do_not_collide.md)
- [signature_accepts_file_path_alias](../../../../../functions/src/llm/agent/service/signature_accepts_file_path_alias.md)