---
type: Rust Method
title: new
resource: src/llm/tools/search_memory.rs#L27-L29
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(store: Arc<MemoryStore>) -> Self`

# Called by

- [build_tool_registry](../../../../../../functions/src/cli/build_tool_registry.md)
- [setup](../../../../../../functions/src/llm/tools/search_memory/setup.md)