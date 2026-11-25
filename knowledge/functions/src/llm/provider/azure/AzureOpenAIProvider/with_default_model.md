---
type: Rust Method
title: with_default_model
resource: src/llm/provider/azure.rs#L79-L82
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/try_create_azure
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_default_model(mut self, model: String) -> Self`

# Calls

- [with_default_model](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_default_model.md)

# Called by

- [try_create_azure](../../../../../../functions/src/llm/provider/factory/try_create_azure.md)