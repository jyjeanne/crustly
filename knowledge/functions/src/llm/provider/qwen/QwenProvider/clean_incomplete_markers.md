---
type: Rust Method
title: clean_incomplete_markers
resource: src/llm/provider/qwen.rs#L601-L620
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_clean_incomplete_markers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn clean_incomplete_markers(&self, text: &str) -> String`

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_clean_incomplete_markers](../../../../../../functions/src/llm/provider/qwen/test_clean_incomplete_markers.md)