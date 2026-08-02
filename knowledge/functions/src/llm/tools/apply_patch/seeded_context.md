---
type: Rust Function
title: seeded_context
resource: src/llm/tools/apply_patch.rs#L537-L544
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/file_read_cache/FileReadCache/record
  - functions/src/llm/tools/file_read_cache/FileFingerprint/of
---

# Signature

`async fn seeded_context(temp_dir: &TempDir, relative_path: &str) -> ToolExecutionContext`

# Calls

- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [of](../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)