---
type: Rust Function
title: clean
resource: src/memory/recall.rs#L211-L214
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/render_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn clean(text: &str) -> String`

# Called by

- [path_memory_text](../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [line](../../../../functions/src/memory/recall/line.md)
- [render_block](../../../../functions/src/memory/recall/render_block.md)