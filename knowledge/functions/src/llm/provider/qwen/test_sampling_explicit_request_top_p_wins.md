---
type: Rust Function
title: test_sampling_explicit_request_top_p_wins
resource: src/llm/provider/qwen.rs#L2444-L2451
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_top_p
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
---

# Signature

`fn test_sampling_explicit_request_top_p_wins()`

# Calls

- [with_top_p](../../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [to_qwen_request](../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)