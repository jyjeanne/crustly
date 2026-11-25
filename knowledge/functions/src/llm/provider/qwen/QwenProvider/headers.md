---
type: Rust Method
title: headers
resource: src/llm/provider/qwen.rs#L246-L267
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/is_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn headers(&self) -> Result<reqwest::header::HeaderMap>`

# Calls

- [is_local](../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)
- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)