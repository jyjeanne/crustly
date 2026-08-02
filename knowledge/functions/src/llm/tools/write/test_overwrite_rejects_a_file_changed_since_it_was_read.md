---
type: Rust Function
title: test_overwrite_rejects_a_file_changed_since_it_was_read
resource: src/llm/tools/write.rs#L399-L428
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/file_read_cache/FileReadCache/record
  - functions/src/llm/tools/file_read_cache/FileFingerprint/of
---

# Signature

`async fn test_overwrite_rejects_a_file_changed_since_it_was_read()`

# Calls

- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [of](../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)