---
type: Rust Method
title: headers
resource: src/llm/provider/qwen.rs#L246-L267
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/QwenProvider/is_local
  - functions/src/db/models/PlanTaskStatus/parse
---

# Signature

`fn headers(&self) -> Result<reqwest::header::HeaderMap>`

# Calls

- [is_local](../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)
- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)