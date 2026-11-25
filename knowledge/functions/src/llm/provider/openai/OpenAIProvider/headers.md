---
type: Rust Method
title: headers
resource: src/llm/provider/openai.rs#L151-L185
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/default_auth_style_still_sends_bearer
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn headers(&self) -> Result<reqwest::header::HeaderMap>`

# Calls

- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [with_api_key_header_sends_api_key_not_bearer](../../../../../../functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer.md)
- [default_auth_style_still_sends_bearer](../../../../../../functions/src/llm/provider/openai/default_auth_style_still_sends_bearer.md)