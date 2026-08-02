---
type: Rust Method
title: clean_incomplete_markers
resource: src/llm/provider/qwen.rs#L601-L620
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/test_clean_incomplete_markers
---

# Signature

`fn clean_incomplete_markers(&self, text: &str) -> String`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_clean_incomplete_markers](../../../../../../functions/src/llm/provider/qwen/test_clean_incomplete_markers.md)