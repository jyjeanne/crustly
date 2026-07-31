---
type: Rust Method
title: extract_thinking
resource: src/llm/provider/qwen.rs#L481-L500
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/find_after
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/test_thinking_extraction
  - functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic
---

# Signature

`fn extract_thinking(&self, text: &str) -> (Option<String>, String)`

# Calls

- [find_after](../../../../../../functions/src/llm/provider/qwen/find_after.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_thinking_extraction](../../../../../../functions/src/llm/provider/qwen/test_thinking_extraction.md)
- [test_thinking_extraction_out_of_order_tags_does_not_panic](../../../../../../functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic.md)