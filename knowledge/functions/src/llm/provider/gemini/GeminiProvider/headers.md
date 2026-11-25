---
type: Rust Method
title: headers
resource: src/llm/provider/gemini.rs#L83-L99
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn headers(&self) -> Result<reqwest::header::HeaderMap>`

# Calls

- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)