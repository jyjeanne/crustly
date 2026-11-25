---
type: Rust Method
title: on_tool_calls
resource: src/llm/agent/middleware.rs#L260-L372
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/tool_call_signature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/has_mutating_capability
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn on_tool_calls(&mut self, state: &RunState) -> Decision`

# Calls

- [tool_call_signature](../../../../../../../functions/src/llm/agent/middleware/tool_call_signature.md)
- [all](../../../../../../../functions/src/memory/MemoryStore/all.md)
- [has_mutating_capability](../../../../../../../functions/src/llm/agent/middleware/has_mutating_capability.md)