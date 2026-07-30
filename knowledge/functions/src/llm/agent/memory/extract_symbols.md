---
type: Rust Function
title: extract_symbols
resource: src/llm/agent/memory.rs#L161-L196
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/memory/CodebaseIndex/index_file
---

# Signature

`fn extract_symbols(file_path: &str, content: &str) -> Vec<CodebaseIndexEntry>`

# Called by

- [index_file](../../../../../functions/src/llm/agent/memory/CodebaseIndex/index_file.md)