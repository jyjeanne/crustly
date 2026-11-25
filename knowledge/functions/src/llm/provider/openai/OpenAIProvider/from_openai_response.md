---
type: Rust Method
title: from_openai_response
resource: src/llm/provider/openai.rs#L360-L482
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from_openai_response(&self, response: OpenAIResponse) -> LLMResponse`

# Calls

- [next](../../../../../../functions/src/tui/events/EventHandler/next.md)
- [extract_think_tags](../../../../../../functions/src/llm/provider/types/extract_think_tags.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)