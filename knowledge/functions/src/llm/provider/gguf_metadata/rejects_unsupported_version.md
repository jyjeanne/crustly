---
type: Rust Function
title: rejects_unsupported_version
resource: src/llm/provider/gguf_metadata.rs#L766-L774
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/write_temp_gguf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/read_gguf_metadata
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rejects_unsupported_version()`

# Calls

- [write_temp_gguf](../../../../../functions/src/llm/provider/gguf_metadata/write_temp_gguf.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)