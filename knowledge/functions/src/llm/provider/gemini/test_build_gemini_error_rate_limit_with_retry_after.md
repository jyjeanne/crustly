---
type: Rust Function
title: test_build_gemini_error_rate_limit_with_retry_after
resource: src/llm/provider/gemini.rs#L1222-L1237
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/build_gemini_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_build_gemini_error_rate_limit_with_retry_after()`

# Calls

- [build_gemini_error](../../../../../functions/src/llm/provider/gemini/build_gemini_error.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)