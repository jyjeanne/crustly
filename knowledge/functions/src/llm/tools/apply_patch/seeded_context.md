---
type: Rust Function
title: seeded_context
resource: src/llm/tools/apply_patch.rs#L537-L544
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileFingerprint/of
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn seeded_context(temp_dir: &TempDir, relative_path: &str) -> ToolExecutionContext`

# Calls

- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [of](../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)