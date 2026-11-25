---
type: Rust Function
title: model_download_typing_filters_suggestions
resource: src/tui/app.rs#L3945-L3961
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/open_model_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_model_download_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn model_download_typing_filters_suggestions()`

# Calls

- [open_model_download](../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)