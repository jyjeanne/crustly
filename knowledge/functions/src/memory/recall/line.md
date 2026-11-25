---
type: Rust Function
title: line
resource: src/memory/recall.rs#L216-L228
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/model/MemoryCategory/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/recall/clean
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/render_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn line(memory: &Memory, with_scope: bool) -> String`

# Calls

- [as_str](../../../../functions/src/memory/model/MemoryCategory/as_str.md)
- [clean](../../../../functions/src/memory/recall/clean.md)

# Called by

- [text_arrives_before_the_response_is_complete](../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [render_block](../../../../functions/src/memory/recall/render_block.md)